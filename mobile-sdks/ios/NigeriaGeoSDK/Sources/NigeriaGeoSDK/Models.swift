import Foundation

/// Data models for the Nigeria Geo SDK

// MARK: - Base Models

/// Represents a Nigerian state
public struct State: Codable, Identifiable, Equatable {
    public let id: UUID
    public let name: String
    public let code: String
    public let capital: String?
    public let region: String?
    public let createdAt: Date
    public let updatedAt: Date
    
    enum CodingKeys: String, CodingKey {
        case id, name, code, capital, region
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

/// Represents a Local Government Area (LGA)
public struct LGA: Codable, Identifiable, Equatable {
    public let id: UUID
    public let name: String
    public let code: String
    public let stateId: UUID
    public let createdAt: Date
    public let updatedAt: Date
    
    enum CodingKeys: String, CodingKey {
        case id, name, code
        case stateId = "state_id"
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

/// Represents a ward
public struct Ward: Codable, Identifiable, Equatable {
    public let id: UUID
    public let name: String
    public let code: String
    public let lgaId: UUID
    public let createdAt: Date
    public let updatedAt: Date
    
    enum CodingKeys: String, CodingKey {
        case id, name, code
        case lgaId = "lga_id"
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

/// Represents a postal code
public struct PostalCode: Codable, Identifiable, Equatable {
    public let id: UUID
    public let code: String
    public let wardId: UUID
    public let latitude: Double?
    public let longitude: Double?
    public let createdAt: Date
    public let updatedAt: Date
    
    /// Computed property for coordinate location
    public var coordinate: Coordinate? {
        guard let latitude = latitude, let longitude = longitude else {
            return nil
        }
        return Coordinate(latitude: latitude, longitude: longitude)
    }
    
    enum CodingKeys: String, CodingKey {
        case id, code, latitude, longitude
        case wardId = "ward_id"
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
}

/// Represents geographic coordinates
public struct Coordinate: Codable, Equatable {
    public let latitude: Double
    public let longitude: Double
    
    public init(latitude: Double, longitude: Double) {
        self.latitude = latitude
        self.longitude = longitude
    }
}

// MARK: - Response Models

/// Generic paginated response
public struct PaginatedResponse<T: Codable>: Codable {
    public let data: [T]
    public let pagination: PaginationMeta
}

/// Pagination metadata
public struct PaginationMeta: Codable {
    public let page: Int
    public let limit: Int
    public let total: Int
    public let totalPages: Int
    public let hasNext: Bool
    public let hasPrev: Bool
    
    enum CodingKeys: String, CodingKey {
        case page, limit, total
        case totalPages = "total_pages"
        case hasNext = "has_next"
        case hasPrev = "has_prev"
    }
}

/// Search result containing all entity types
public struct SearchResult: Codable {
    public let states: [State]
    public let lgas: [LGA]
    public let wards: [Ward]
    public let postalCodes: [PostalCode]
    
    enum CodingKeys: String, CodingKey {
        case states, lgas, wards
        case postalCodes = "postal_codes"
    }
}

// MARK: - Address Validation Models

/// Request for address validation
public struct AddressValidationRequest: Codable {
    public let state: String
    public let lga: String
    public let ward: String?
    public let postalCode: String?
    public let streetAddress: String?
    
    public init(
        state: String,
        lga: String,
        ward: String? = nil,
        postalCode: String? = nil,
        streetAddress: String? = nil
    ) {
        self.state = state
        self.lga = lga
        self.ward = ward
        self.postalCode = postalCode
        self.streetAddress = streetAddress
    }
    
    enum CodingKeys: String, CodingKey {
        case state, lga, ward
        case postalCode = "postal_code"
        case streetAddress = "street_address"
    }
}

/// Response for address validation
public struct AddressValidationResponse: Codable {
    public let isValid: Bool
    public let confidence: Double
    public let normalizedAddress: NormalizedAddress?
    public let suggestions: [AddressSuggestion]
    public let errors: [ValidationError]
    
    enum CodingKeys: String, CodingKey {
        case isValid = "is_valid"
        case confidence
        case normalizedAddress = "normalized_address"
        case suggestions, errors
    }
}

/// Normalized address structure
public struct NormalizedAddress: Codable {
    public let state: String
    public let lga: String
    public let ward: String?
    public let postalCode: String?
    public let streetAddress: String?
    public let coordinates: Coordinate?
    
    enum CodingKeys: String, CodingKey {
        case state, lga, ward, coordinates
        case postalCode = "postal_code"
        case streetAddress = "street_address"
    }
}

/// Address suggestion for corrections
public struct AddressSuggestion: Codable {
    public let type: SuggestionType
    public let field: String
    public let currentValue: String
    public let suggestedValue: String
    public let confidence: Double
    
    enum CodingKeys: String, CodingKey {
        case type, field, confidence
        case currentValue = "current_value"
        case suggestedValue = "suggested_value"
    }
}

/// Types of address suggestions
public enum SuggestionType: String, Codable, CaseIterable {
    case correction = "correction"
    case completion = "completion"
    case alternative = "alternative"
}

/// Validation error details
public struct ValidationError: Codable {
    public let field: String
    public let message: String
    public let code: String
}

// MARK: - Search Helper Models

/// Unified search result item for UI display
public struct SearchResultItem {
    public let type: SearchResultType
    public let id: UUID
    public let name: String
    public let subtitle: String?
    public let data: Any
    
    public init(type: SearchResultType, data: Any) {
        self.type = type
        
        switch data {
        case let state as State:
            self.id = state.id
            self.name = state.name
            self.subtitle = state.capital
            self.data = state
        case let lga as LGA:
            self.id = lga.id
            self.name = lga.name
            self.subtitle = "LGA"
            self.data = lga
        case let ward as Ward:
            self.id = ward.id
            self.name = ward.name
            self.subtitle = "Ward"
            self.data = ward
        case let postalCode as PostalCode:
            self.id = postalCode.id
            self.name = postalCode.code
            self.subtitle = "Postal Code"
            self.data = postalCode
        default:
            fatalError("Unsupported data type")
        }
    }
}

/// Types of search results
public enum SearchResultType: String, CaseIterable {
    case state = "state"
    case lga = "lga"
    case ward = "ward"
    case postalCode = "postal_code"
    
    public var displayName: String {
        switch self {
        case .state:
            return "State"
        case .lga:
            return "LGA"
        case .ward:
            return "Ward"
        case .postalCode:
            return "Postal Code"
        }
    }
}

// MARK: - Performance Monitoring Models

/// Performance metrics for monitoring
public struct PerformanceMetrics {
    public let averageResponseTime: Double // in milliseconds
    public let cacheHitRate: Double // percentage
    public let totalRequests: Int
    public let failedRequests: Int
    public let lastRequestTime: Date?
    
    public var successRate: Double {
        guard totalRequests > 0 else { return 0.0 }
        return Double(totalRequests - failedRequests) / Double(totalRequests) * 100.0
    }
}

/// Cache statistics
public struct CacheStatistics {
    public let itemCount: Int
    public let sizeInBytes: Int
    public let hitCount: Int
    public let missCount: Int
    
    public var hitRate: Double {
        let totalAccess = hitCount + missCount
        guard totalAccess > 0 else { return 0.0 }
        return Double(hitCount) / Double(totalAccess) * 100.0
    }
    
    public var sizeInMegabytes: Double {
        return Double(sizeInBytes) / (1024.0 * 1024.0)
    }
}