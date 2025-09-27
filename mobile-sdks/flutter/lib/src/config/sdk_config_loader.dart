import 'package:yaml/yaml.dart';

/// Configuration loader for the Nigeria Geo SDK
/// Provides built-in defaults, no external configuration required
class SDKConfigLoader {
  static Map<String, dynamic>? _config;

  /// Initialize configuration with built-in defaults
  /// Optionally loads from YAML file to override defaults
  static Future<Map<String, dynamic>> loadConfig() async {
    if (_config != null) return _config!;

    // Start with built-in defaults - SDK works out of the box!
    _config = _getDefaultConfig();
    return _config!;
  }

  /// Get configuration synchronously (uses defaults if not loaded)
  static Map<String, dynamic> getConfig() {
    return _config ?? _getDefaultConfig();
  }

  /// Get default configuration if YAML loading fails
  static Map<String, dynamic> _getDefaultConfig() {
    return {
      'api': {
        'base_url': 'http://localhost:3000',
        'timeout': 30000,
      },
      'cache': {
        'enabled': true,
        'expiration_hours': 24,
        'max_size_mb': 50,
      },
      'logging': {
        'enabled': true,
        'level': 'info',
      },
      'location': {
        'request_permission': true,
        'accuracy': 'high',
      },
    };
  }

  /// Convert YamlMap to regular Map recursively
  static Map<String, dynamic> _convertYamlMap(YamlMap yamlMap) {
    final Map<String, dynamic> result = {};

    for (final entry in yamlMap.entries) {
      final key = entry.key.toString();
      final value = entry.value;

      if (value is YamlMap) {
        result[key] = _convertYamlMap(value);
      } else if (value is YamlList) {
        result[key] = value.toList();
      } else {
        result[key] = value;
      }
    }

    return result;
  }

  /// Get API base URL
  static String getBaseUrl() {
    final config = getConfig();
    return config['api']?['base_url'] ?? 'http://localhost:3000';
  }

  /// Get API timeout
  static int getTimeout() {
    final config = getConfig();
    return config['api']?['timeout'] ?? 30000;
  }

  /// Check if caching is enabled
  static bool isCacheEnabled() {
    final config = getConfig();
    return config['cache']?['enabled'] ?? true;
  }

  /// Get cache expiration in hours
  static int getCacheExpirationHours() {
    return _config?['cache']?['expiration_hours'] ?? 24;
  }

  /// Get maximum cache size in MB
  static int getMaxCacheSizeMB() {
    final config = getConfig();
    return config['cache']?['max_size_mb'] ?? 50;
  }

  /// Check if logging is enabled
  static bool isLoggingEnabled() {
    final config = getConfig();
    return config['logging']?['enabled'] ?? true;
  }

  /// Get logging level
  static String getLogLevel() {
    final config = getConfig();
    return config['logging']?['level'] ?? 'info';
  }

  /// Check if location permission should be requested
  static bool shouldRequestLocationPermission() {
    final config = getConfig();
    return config['location']?['request_permission'] ?? true;
  }

  /// Get location accuracy setting
  static String getLocationAccuracy() {
    final config = getConfig();
    return config['location']?['accuracy'] ?? 'high';
  }

  /// Update base URL at runtime (for development/testing)
  static void updateBaseUrl(String newBaseUrl) {
    final config = getConfig();
    _config = {...config};
    _config!['api']['base_url'] = newBaseUrl;
  }

  /// Reset configuration (useful for testing)
  static void reset() {
    _config = null;
  }
}
