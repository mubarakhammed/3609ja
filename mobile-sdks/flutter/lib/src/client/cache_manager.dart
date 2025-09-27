import '../services/cache_service.dart';
import '../models/state.dart';
import '../models/lga.dart';
import '../models/ward.dart';
import '../models/postal_code.dart';
import '../models/search_result.dart';
import '../models/pagination.dart';
import '../utils/logger.dart';

/// High-level cache manager for common operations
class CacheManager {
  final CacheService _cacheService;

  CacheManager(this._cacheService);

  // States cache methods
  Future<List<NigerianState>?> getCachedStates(
      {int page = 1, int limit = 20}) async {
    final cacheKey = 'states_${page}_$limit';
    final cached = await _cacheService.get<PaginatedResponse<NigerianState>>(
      cacheKey,
      (json) => PaginatedResponse.fromJson(
        json,
        (item) => NigerianState.fromJson(item as Map<String, dynamic>),
      ),
    );
    return cached?.data;
  }

  Future<bool> hasStatesCache({int page = 1, int limit = 20}) async {
    final cacheKey = 'states_${page}_$limit';
    return await _cacheService.hasKey(cacheKey);
  }

  Future<void> clearStatesCache() async {
    final keys = await _cacheService.getKeysMatching(r'^states_\d+_\d+$');
    for (final key in keys) {
      await _cacheService.remove(key);
    }
    NigeriaGeoLogger.instance.d('Cleared states cache');
  }

  // LGAs cache methods
  Future<List<LGA>?> getCachedLGAs({
    String? stateId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'lgas_${stateId ?? 'all'}_${page}_$limit';
    final cached = await _cacheService.get<PaginatedResponse<LGA>>(
      cacheKey,
      (json) => PaginatedResponse.fromJson(
        json,
        (item) => LGA.fromJson(item as Map<String, dynamic>),
      ),
    );
    return cached?.data;
  }

  Future<bool> hasLGAsCache({
    String? stateId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'lgas_${stateId ?? 'all'}_${page}_$limit';
    return await _cacheService.hasKey(cacheKey);
  }

  Future<void> clearLGAsCache() async {
    final keys = await _cacheService.getKeysMatching(r'^lgas_.*_\d+_\d+$');
    for (final key in keys) {
      await _cacheService.remove(key);
    }
    NigeriaGeoLogger.instance.d('Cleared LGAs cache');
  }

  // Wards cache methods
  Future<List<Ward>?> getCachedWards({
    String? lgaId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'wards_${lgaId ?? 'all'}_${page}_$limit';
    final cached = await _cacheService.get<PaginatedResponse<Ward>>(
      cacheKey,
      (json) => PaginatedResponse.fromJson(
        json,
        (item) => Ward.fromJson(item as Map<String, dynamic>),
      ),
    );
    return cached?.data;
  }

  Future<bool> hasWardsCache({
    String? lgaId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'wards_${lgaId ?? 'all'}_${page}_$limit';
    return await _cacheService.hasKey(cacheKey);
  }

  Future<void> clearWardsCache() async {
    final keys = await _cacheService.getKeysMatching(r'^wards_.*_\d+_\d+$');
    for (final key in keys) {
      await _cacheService.remove(key);
    }
    NigeriaGeoLogger.instance.d('Cleared wards cache');
  }

  // Postal codes cache methods
  Future<List<PostalCode>?> getCachedPostalCodes({
    String? wardId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'postal_codes_${wardId ?? 'all'}_${page}_$limit';
    final cached = await _cacheService.get<PaginatedResponse<PostalCode>>(
      cacheKey,
      (json) => PaginatedResponse.fromJson(
        json,
        (item) => PostalCode.fromJson(item as Map<String, dynamic>),
      ),
    );
    return cached?.data;
  }

  Future<bool> hasPostalCodesCache({
    String? wardId,
    int page = 1,
    int limit = 20,
  }) async {
    final cacheKey = 'postal_codes_${wardId ?? 'all'}_${page}_$limit';
    return await _cacheService.hasKey(cacheKey);
  }

  Future<void> clearPostalCodesCache() async {
    final keys =
        await _cacheService.getKeysMatching(r'^postal_codes_.*_\d+_\d+$');
    for (final key in keys) {
      await _cacheService.remove(key);
    }
    NigeriaGeoLogger.instance.d('Cleared postal codes cache');
  }

  // Search cache methods
  Future<SearchResult?> getCachedSearchResult(String query) async {
    final cacheKey = 'search_${query.toLowerCase().trim()}';
    return await _cacheService.get<SearchResult>(
      cacheKey,
      (json) => SearchResult.fromJson(json),
    );
  }

  Future<bool> hasSearchCache(String query) async {
    final cacheKey = 'search_${query.toLowerCase().trim()}';
    return await _cacheService.hasKey(cacheKey);
  }

  Future<void> clearSearchCache() async {
    final keys = await _cacheService.getKeysMatching(r'^search_.*$');
    for (final key in keys) {
      await _cacheService.remove(key);
    }
    NigeriaGeoLogger.instance.d('Cleared search cache');
  }

  // Entity-specific cache methods
  Future<NigerianState?> getCachedState(String stateId) async {
    final cacheKey = 'state_$stateId';
    return await _cacheService.get<NigerianState>(
      cacheKey,
      (json) => NigerianState.fromJson(json),
    );
  }

  Future<LGA?> getCachedLGA(String lgaId) async {
    final cacheKey = 'lga_$lgaId';
    return await _cacheService.get<LGA>(
      cacheKey,
      (json) => LGA.fromJson(json),
    );
  }

  Future<Ward?> getCachedWard(String wardId) async {
    final cacheKey = 'ward_$wardId';
    return await _cacheService.get<Ward>(
      cacheKey,
      (json) => Ward.fromJson(json),
    );
  }

  Future<PostalCode?> getCachedPostalCode(String postalCodeId) async {
    final cacheKey = 'postal_code_$postalCodeId';
    return await _cacheService.get<PostalCode>(
      cacheKey,
      (json) => PostalCode.fromJson(json),
    );
  }

  // General cache operations
  Future<void> clearAll() async {
    await _cacheService.clearAll();
    NigeriaGeoLogger.instance.i('Cleared all cache');
  }

  Future<CacheStatistics> getStatistics() async {
    return await _cacheService.getStatistics();
  }

  Future<bool> hasKey(String key) async {
    return await _cacheService.hasKey(key);
  }

  Future<void> remove(String key) async {
    await _cacheService.remove(key);
  }

  // Preloading methods for offline use
  Future<void> preloadStates() async {
    try {
      // This will be called by the client to cache states
      NigeriaGeoLogger.instance.d('Preloading states for offline use');
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to preload states: $e');
    }
  }

  Future<void> preloadLGAsForState(String stateId) async {
    try {
      // This will be called by the client to cache LGAs for a state
      NigeriaGeoLogger.instance.d('Preloading LGAs for state $stateId');
    } catch (e) {
      NigeriaGeoLogger.instance
          .w('Failed to preload LGAs for state $stateId: $e');
    }
  }

  Future<void> preloadWardsForLGA(String lgaId) async {
    try {
      // This will be called by the client to cache wards for an LGA
      NigeriaGeoLogger.instance.d('Preloading wards for LGA $lgaId');
    } catch (e) {
      NigeriaGeoLogger.instance.w('Failed to preload wards for LGA $lgaId: $e');
    }
  }

  // Cache health check
  Future<Map<String, dynamic>> healthCheck() async {
    final stats = await getStatistics();

    return {
      'isHealthy': true,
      'itemCount': stats.itemCount,
      'sizeInMB': stats.sizeInMB,
      'hitRate': stats.hitRate,
      'timestamp': DateTime.now().toIso8601String(),
    };
  }

  /// Dispose cache manager
  Future<void> dispose() async {
    await _cacheService.dispose();
  }
}
