import 'dart:convert';

import 'package:hive_flutter/hive_flutter.dart';

import '../exceptions/nigeria_geo_exception.dart';
import '../utils/logger.dart';

/// Cache service for storing API responses locally
class CacheService {
  static const String _boxName = 'nigeria_geo_cache';
  static const String _metaBoxName = 'nigeria_geo_cache_meta';

  final bool enableCaching;
  final Duration expiration;
  final int maxSize;

  Box<String>? _cacheBox;
  Box<Map<dynamic, dynamic>>? _metaBox;
  bool _isInitialized = false;

  CacheService({
    required this.enableCaching,
    required this.expiration,
    required this.maxSize,
  });

  /// Initialize the cache service
  Future<void> initialize() async {
    if (!enableCaching || _isInitialized) return;

    try {
      _cacheBox = await Hive.openBox<String>(_boxName);
      _metaBox = await Hive.openBox<Map<dynamic, dynamic>>(_metaBoxName);
      _isInitialized = true;

      // Clean expired entries on initialization
      await _cleanExpiredEntries();

      NigeriaGeoLogger.instance.d('Cache service initialized');
    } catch (e) {
      NigeriaGeoLogger.instance.e('Failed to initialize cache service: $e');
      throw CacheException('Failed to initialize cache: $e');
    }
  }

  /// Get cached data
  Future<T?> get<T>(
    String key,
    T Function(Map<String, dynamic>) fromJson,
  ) async {
    if (!enableCaching || !_isInitialized) return null;

    try {
      final cachedData = _cacheBox?.get(key);
      if (cachedData == null) return null;

      // Check expiration
      final meta = _metaBox?.get(key);
      if (meta != null) {
        final timestamp =
            DateTime.fromMillisecondsSinceEpoch(meta['timestamp'] as int);
        if (DateTime.now().difference(timestamp) > expiration) {
          await remove(key);
          return null;
        }
      }

      final jsonData = jsonDecode(cachedData) as Map<String, dynamic>;
      return fromJson(jsonData);
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to get cached data for key $key: $e');
      return null;
    }
  }

  /// Set cached data
  Future<void> set(String key, Map<String, dynamic> data) async {
    if (!enableCaching || !_isInitialized) return;

    try {
      // Check cache size before adding
      await _ensureCacheSize();

      final jsonString = jsonEncode(data);
      await _cacheBox?.put(key, jsonString);

      // Store metadata
      await _metaBox?.put(key, {
        'timestamp': DateTime.now().millisecondsSinceEpoch,
        'size': jsonString.length,
      });

      NigeriaGeoLogger.instance.d('Cached data for key: $key');
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to cache data for key $key: $e');
      throw CacheException('Failed to cache data: $e');
    }
  }

  /// Remove cached data
  Future<void> remove(String key) async {
    if (!enableCaching || !_isInitialized) return;

    try {
      await _cacheBox?.delete(key);
      await _metaBox?.delete(key);
      NigeriaGeoLogger.instance.d('Removed cached data for key: $key');
    } catch (e) {
      NigeriaGeoLogger.instance
          .w('Failed to remove cached data for key $key: $e');
    }
  }

  /// Clear all cached data
  Future<void> clearAll() async {
    if (!enableCaching || !_isInitialized) return;

    try {
      await _cacheBox?.clear();
      await _metaBox?.clear();
      NigeriaGeoLogger.instance.i('Cleared all cached data');
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to clear cache: $e');
      throw CacheException('Failed to clear cache: $e');
    }
  }

  /// Get cache statistics
  Future<CacheStatistics> getStatistics() async {
    if (!enableCaching || !_isInitialized) {
      return CacheStatistics(
        itemCount: 0,
        totalSize: 0,
        hitCount: 0,
        missCount: 0,
      );
    }

    try {
      final itemCount = _cacheBox?.length ?? 0;
      var totalSize = 0;

      // Calculate total size from metadata
      for (final key in _metaBox?.keys ?? <String>[]) {
        final meta = _metaBox?.get(key);
        if (meta != null) {
          totalSize += (meta['size'] as int?) ?? 0;
        }
      }

      return CacheStatistics(
        itemCount: itemCount,
        totalSize: totalSize,
        hitCount: 0, // TODO: Implement hit/miss tracking
        missCount: 0,
      );
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to get cache statistics: $e');
      return CacheStatistics(
        itemCount: 0,
        totalSize: 0,
        hitCount: 0,
        missCount: 0,
      );
    }
  }

  /// Check if cache has data for key
  Future<bool> hasKey(String key) async {
    if (!enableCaching || !_isInitialized) return false;
    return _cacheBox?.containsKey(key) ?? false;
  }

  /// Get cache keys matching pattern
  Future<List<String>> getKeysMatching(String pattern) async {
    if (!enableCaching || !_isInitialized) return [];

    try {
      final allKeys = _cacheBox?.keys.cast<String>() ?? <String>[];
      final regex = RegExp(pattern);
      return allKeys.where((key) => regex.hasMatch(key)).toList();
    } catch (e) {
      NigeriaGeoLogger.instance
          .w('Failed to get keys matching pattern $pattern: $e');
      return [];
    }
  }

  /// Clean expired entries
  Future<void> _cleanExpiredEntries() async {
    if (!_isInitialized) return;

    try {
      final keysToRemove = <String>[];
      final now = DateTime.now();

      for (final key in _metaBox?.keys.cast<String>() ?? <String>[]) {
        final meta = _metaBox?.get(key);
        if (meta != null) {
          final timestamp =
              DateTime.fromMillisecondsSinceEpoch(meta['timestamp'] as int);
          if (now.difference(timestamp) > expiration) {
            keysToRemove.add(key);
          }
        }
      }

      for (final key in keysToRemove) {
        await remove(key);
      }

      if (keysToRemove.isNotEmpty) {
        NigeriaGeoLogger.instance
            .d('Cleaned ${keysToRemove.length} expired cache entries');
      }
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to clean expired entries: $e');
    }
  }

  /// Ensure cache doesn't exceed max size
  Future<void> _ensureCacheSize() async {
    if (!_isInitialized) return;

    try {
      final stats = await getStatistics();

      if (stats.totalSize > maxSize) {
        // Remove oldest entries until we're under the limit
        final sortedKeys = <MapEntry<String, int>>[];

        for (final key in _metaBox?.keys.cast<String>() ?? <String>[]) {
          final meta = _metaBox?.get(key);
          if (meta != null) {
            sortedKeys.add(MapEntry(key, meta['timestamp'] as int));
          }
        }

        sortedKeys.sort((a, b) => a.value.compareTo(b.value));

        // Remove oldest entries until size is acceptable
        var currentSize = stats.totalSize;
        for (final entry in sortedKeys) {
          if (currentSize <= maxSize * 0.8) break; // Leave some headroom

          final meta = _metaBox?.get(entry.key);
          if (meta != null) {
            currentSize -= (meta['size'] as int?) ?? 0;
            await remove(entry.key);
          }
        }

        NigeriaGeoLogger.instance.d('Cache cleanup completed');
      }
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to ensure cache size: $e');
    }
  }

  /// Dispose cache resources
  Future<void> dispose() async {
    try {
      await _cacheBox?.close();
      await _metaBox?.close();
      _isInitialized = false;
      NigeriaGeoLogger.instance.d('Cache service disposed');
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to dispose cache service: $e');
    }
  }
}

/// Cache statistics data class
class CacheStatistics {
  final int itemCount;
  final int totalSize;
  final int hitCount;
  final int missCount;

  const CacheStatistics({
    required this.itemCount,
    required this.totalSize,
    required this.hitCount,
    required this.missCount,
  });

  /// Get cache size in megabytes
  double get sizeInMB => totalSize / (1024 * 1024);

  /// Get cache hit rate (0.0 to 1.0)
  double get hitRate {
    final total = hitCount + missCount;
    return total > 0 ? hitCount / total : 0.0;
  }

  @override
  String toString() =>
      'CacheStatistics(items: $itemCount, size: ${sizeInMB.toStringAsFixed(2)}MB, hitRate: ${(hitRate * 100).toStringAsFixed(1)}%)';
}
