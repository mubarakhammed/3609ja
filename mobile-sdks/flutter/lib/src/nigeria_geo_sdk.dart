import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:http/http.dart' as http;

import 'client/nigeria_geo_client.dart';
import 'client/nigeria_geo_config.dart';
import 'client/cache_manager.dart';
import 'services/cache_service.dart';
import 'utils/logger.dart';
import 'config/sdk_config_loader.dart';

/// Main SDK class for Nigeria Geo API
class NigeriaGeoSDK {
  static NigeriaGeoSDK? _instance;
  static NigeriaGeoClient? _client;
  static NigeriaGeoConfig? _config;
  static CacheManager? _cacheManager;

  NigeriaGeoSDK._();

  /// Initialize the SDK with configuration
  /// No configuration required - uses sensible defaults
  static Future<void> initialize([NigeriaGeoConfig? config]) async {
    if (_instance != null) {
      throw StateError('NigeriaGeoSDK is already initialized');
    }

    // Load configuration (uses built-in defaults, optionally overridden by config.yaml)
    await SDKConfigLoader.loadConfig();

    // Use provided config or create default
    if (config == null) {
      config = NigeriaGeoConfig(
        baseUrl: SDKConfigLoader.getBaseUrl(),
        timeout: Duration(milliseconds: SDKConfigLoader.getTimeout()),
        enableCaching: SDKConfigLoader.isCacheEnabled(),
        enableLogging: SDKConfigLoader.isLoggingEnabled(),
      );
    }

    _config = config;
    _instance = NigeriaGeoSDK._();

    // Initialize logger
    NigeriaGeoLogger.initialize(config.enableLogging);

    // Initialize Hive for caching if enabled
    if (config.enableCaching) {
      if (!kIsWeb) {
        await Hive.initFlutter();
      }
    }

    // Create cache service
    final cacheService = CacheService(
      enableCaching: config.enableCaching,
      expiration: config.cacheExpiration,
      maxSize: config.maxCacheSize,
    );
    await cacheService.initialize();

    // Create HTTP client
    final httpClient = http.Client();

    // Create cache manager
    _cacheManager = CacheManager(cacheService);

    // Create API client
    _client = NigeriaGeoClient.create(config, httpClient, cacheService);

    NigeriaGeoLogger.instance.i('NigeriaGeoSDK initialized successfully');
  }

  /// Get the configured client instance
  static NigeriaGeoClient get client {
    if (_client == null) {
      throw StateError(
          'NigeriaGeoSDK not initialized. Call initialize() first.');
    }
    return _client!;
  }

  /// Get the current configuration
  static NigeriaGeoConfig get config {
    if (_config == null) {
      throw StateError(
          'NigeriaGeoSDK not initialized. Call initialize() first.');
    }
    return _config!;
  }

  /// Get the cache manager
  static CacheManager get cacheManager {
    if (_cacheManager == null) {
      throw StateError(
          'NigeriaGeoSDK not initialized. Call initialize() first.');
    }
    return _cacheManager!;
  }

  /// Check if SDK is initialized
  static bool get isInitialized => _instance != null;

  /// Create a Nigeria Geo client without initializing the singleton
  /// This is a simpler alternative that doesn't require initialization
  static Future<NigeriaGeoClient> createClient() async {
    // Load configuration (uses built-in defaults)
    await SDKConfigLoader.loadConfig();

    // Create configuration
    final config = NigeriaGeoConfig(
      baseUrl: SDKConfigLoader.getBaseUrl(),
      timeout: Duration(milliseconds: SDKConfigLoader.getTimeout()),
      enableCaching: SDKConfigLoader.isCacheEnabled(),
      enableLogging: SDKConfigLoader.isLoggingEnabled(),
    );

    // Create cache service
    final cacheService = CacheService(
      enableCaching: config.enableCaching,
      expiration: Duration(hours: SDKConfigLoader.getCacheExpirationHours()),
      maxSize: SDKConfigLoader.getMaxCacheSizeMB(),
    );

    // Create HTTP client
    final httpClient = http.Client();

    // Create and return client
    return NigeriaGeoClient.create(config, httpClient, cacheService);
  }

  /// Dispose and cleanup resources
  static Future<void> dispose() async {
    if (_client != null) {
      _client!.dispose();
      _client = null;
    }

    if (_cacheManager != null) {
      await _cacheManager!.dispose();
      _cacheManager = null;
    }

    _config = null;
    _instance = null;

    NigeriaGeoLogger.instance.i('NigeriaGeoSDK disposed');
  }

  /// Reset SDK (for testing purposes)
  @visibleForTesting
  static Future<void> reset() async {
    await dispose();
  }
}
