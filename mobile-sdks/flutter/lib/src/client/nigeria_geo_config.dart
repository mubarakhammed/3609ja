class NigeriaGeoConfig {
  /// Base URL for the Nigeria Geo API
  final String baseUrl;

  /// API key for authentication (optional)
  final String? apiKey;

  /// Whether to enable caching
  final bool enableCaching;

  /// Cache expiration duration
  final Duration cacheExpiration;

  /// Maximum cache size in bytes
  final int maxCacheSize;

  /// Request timeout duration
  final Duration timeout;

  /// Whether to enable detailed logging
  final bool enableLogging;

  /// Custom user agent for HTTP requests
  final String? userAgent;

  /// Whether to enable retry on network failures
  final bool enableRetry;

  /// Maximum number of retry attempts
  final int maxRetries;

  /// Base delay between retry attempts
  final Duration retryDelay;

  const NigeriaGeoConfig({
    required this.baseUrl,
    this.apiKey,
    this.enableCaching = true,
    this.cacheExpiration =
        const Duration(hours: 6), // Shorter cache for fast API
    this.maxCacheSize = 25 * 1024 * 1024, // 25MB - reduced cache size
    this.timeout = const Duration(seconds: 5), // Optimized timeout
    this.enableLogging = false,
    this.userAgent,
    this.enableRetry = true,
    this.maxRetries = 2, // Fewer retries for fast API
    this.retryDelay = const Duration(milliseconds: 500), // Faster retry
  });

  /// Creates a config for development environment
  factory NigeriaGeoConfig.development({
    required String baseUrl,
    String? apiKey,
  }) {
    return NigeriaGeoConfig(
      baseUrl: baseUrl,
      apiKey: apiKey,
      enableCaching: true,
      cacheExpiration: const Duration(minutes: 30),
      enableLogging: true,
      timeout: const Duration(seconds: 10),
      maxRetries: 1,
    );
  }

  /// Creates a config for production environment
  factory NigeriaGeoConfig.production({
    required String baseUrl,
    required String apiKey,
  }) {
    return NigeriaGeoConfig(
      baseUrl: baseUrl,
      apiKey: apiKey,
      enableCaching: true,
      cacheExpiration: const Duration(hours: 24),
      enableLogging: false,
      timeout: const Duration(seconds: 30),
      maxRetries: 3,
      retryDelay: const Duration(seconds: 2),
    );
  }

  /// Creates a config with caching disabled
  factory NigeriaGeoConfig.noCaching({
    required String baseUrl,
    String? apiKey,
  }) {
    return NigeriaGeoConfig(
      baseUrl: baseUrl,
      apiKey: apiKey,
      enableCaching: false,
      enableLogging: false,
    );
  }

  /// Copy config with modified values
  NigeriaGeoConfig copyWith({
    String? baseUrl,
    String? apiKey,
    bool? enableCaching,
    Duration? cacheExpiration,
    int? maxCacheSize,
    Duration? timeout,
    bool? enableLogging,
    String? userAgent,
    bool? enableRetry,
    int? maxRetries,
    Duration? retryDelay,
  }) {
    return NigeriaGeoConfig(
      baseUrl: baseUrl ?? this.baseUrl,
      apiKey: apiKey ?? this.apiKey,
      enableCaching: enableCaching ?? this.enableCaching,
      cacheExpiration: cacheExpiration ?? this.cacheExpiration,
      maxCacheSize: maxCacheSize ?? this.maxCacheSize,
      timeout: timeout ?? this.timeout,
      enableLogging: enableLogging ?? this.enableLogging,
      userAgent: userAgent ?? this.userAgent,
      enableRetry: enableRetry ?? this.enableRetry,
      maxRetries: maxRetries ?? this.maxRetries,
      retryDelay: retryDelay ?? this.retryDelay,
    );
  }

  @override
  String toString() {
    return 'NigeriaGeoConfig('
        'baseUrl: $baseUrl, '
        'hasApiKey: ${apiKey != null}, '
        'enableCaching: $enableCaching, '
        'cacheExpiration: $cacheExpiration, '
        'timeout: $timeout'
        ')';
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is NigeriaGeoConfig &&
        other.baseUrl == baseUrl &&
        other.apiKey == apiKey &&
        other.enableCaching == enableCaching &&
        other.cacheExpiration == cacheExpiration &&
        other.maxCacheSize == maxCacheSize &&
        other.timeout == timeout &&
        other.enableLogging == enableLogging &&
        other.userAgent == userAgent &&
        other.enableRetry == enableRetry &&
        other.maxRetries == maxRetries &&
        other.retryDelay == retryDelay;
  }

  @override
  int get hashCode {
    return Object.hash(
      baseUrl,
      apiKey,
      enableCaching,
      cacheExpiration,
      maxCacheSize,
      timeout,
      enableLogging,
      userAgent,
      enableRetry,
      maxRetries,
      retryDelay,
    );
  }
}
