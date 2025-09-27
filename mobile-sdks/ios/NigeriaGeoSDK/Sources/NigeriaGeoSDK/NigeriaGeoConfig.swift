import Foundation

/// Configuration for the Nigeria Geo SDK
public struct NigeriaGeoConfig {
    
    // MARK: - Properties
    
    /// Base URL for the API
    public let baseURL: String
    
    /// Optional API key for authentication
    public let apiKey: String?
    
    /// Whether to enable local caching
    public let enableCaching: Bool
    
    /// Cache expiration duration
    public let cacheExpiration: CacheExpiration
    
    /// Maximum cache size
    public let maxCacheSize: CacheSize
    
    /// Request timeout in seconds
    public let requestTimeout: TimeInterval
    
    /// Whether to enable performance monitoring
    public let enablePerformanceMonitoring: Bool
    
    /// Custom HTTP headers
    public let customHeaders: [String: String]
    
    // MARK: - Initialization
    
    /// Initialize SDK configuration
    /// - Parameters:
    ///   - baseURL: API base URL
    ///   - apiKey: Optional API key
    ///   - enableCaching: Enable local caching (default: true)
    ///   - cacheExpiration: Cache expiration duration (default: 24 hours)
    ///   - maxCacheSize: Maximum cache size (default: 50MB)
    ///   - requestTimeout: Request timeout (default: 30 seconds)
    ///   - enablePerformanceMonitoring: Enable performance monitoring (default: false)
    ///   - customHeaders: Custom HTTP headers (default: empty)
    public init(
        baseURL: String,
        apiKey: String? = nil,
        enableCaching: Bool = true,
        cacheExpiration: CacheExpiration = .hours(24),
        maxCacheSize: CacheSize = .megabytes(50),
        requestTimeout: TimeInterval = 30.0,
        enablePerformanceMonitoring: Bool = false,
        customHeaders: [String: String] = [:]
    ) {
        self.baseURL = baseURL
        self.apiKey = apiKey
        self.enableCaching = enableCaching
        self.cacheExpiration = cacheExpiration
        self.maxCacheSize = maxCacheSize
        self.requestTimeout = requestTimeout
        self.enablePerformanceMonitoring = enablePerformanceMonitoring
        self.customHeaders = customHeaders
    }
}

// MARK: - Cache Configuration

/// Cache expiration options
public enum CacheExpiration {
    case minutes(Int)
    case hours(Int)
    case days(Int)
    case never
    
    var timeInterval: TimeInterval {
        switch self {
        case .minutes(let minutes):
            return TimeInterval(minutes * 60)
        case .hours(let hours):
            return TimeInterval(hours * 3600)
        case .days(let days):
            return TimeInterval(days * 24 * 3600)
        case .never:
            return TimeInterval.greatestFiniteMagnitude
        }
    }
}

/// Cache size options
public enum CacheSize {
    case kilobytes(Int)
    case megabytes(Int)
    case gigabytes(Int)
    
    var bytes: Int {
        switch self {
        case .kilobytes(let kb):
            return kb * 1024
        case .megabytes(let mb):
            return mb * 1024 * 1024
        case .gigabytes(let gb):
            return gb * 1024 * 1024 * 1024
        }
    }
}

// MARK: - Default Configurations

public extension NigeriaGeoConfig {
    
    /// Default configuration for development
    static func development(baseURL: String, apiKey: String? = nil) -> NigeriaGeoConfig {
        return NigeriaGeoConfig(
            baseURL: baseURL,
            apiKey: apiKey,
            enableCaching: true,
            cacheExpiration: .hours(1), // Shorter cache for development
            maxCacheSize: .megabytes(10),
            requestTimeout: 15.0,
            enablePerformanceMonitoring: true
        )
    }
    
    /// Default configuration for production
    static func production(baseURL: String, apiKey: String? = nil) -> NigeriaGeoConfig {
        return NigeriaGeoConfig(
            baseURL: baseURL,
            apiKey: apiKey,
            enableCaching: true,
            cacheExpiration: .hours(24),
            maxCacheSize: .megabytes(100),
            requestTimeout: 30.0,
            enablePerformanceMonitoring: false
        )
    }
    
    /// Configuration with minimal caching for testing
    static func testing(baseURL: String, apiKey: String? = nil) -> NigeriaGeoConfig {
        return NigeriaGeoConfig(
            baseURL: baseURL,
            apiKey: apiKey,
            enableCaching: false,
            cacheExpiration: .minutes(1),
            maxCacheSize: .megabytes(1),
            requestTimeout: 10.0,
            enablePerformanceMonitoring: true
        )
    }
}