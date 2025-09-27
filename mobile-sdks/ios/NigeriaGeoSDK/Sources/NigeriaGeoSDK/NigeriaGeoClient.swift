import Foundation

/// Main SDK client for Nigeria Geo API
@available(iOS 12.0, macOS 10.15, tvOS 12.0, watchOS 6.0, *)
public class NigeriaGeoClient {
    
    // MARK: - Properties
    
    private let config: NigeriaGeoConfig
    private let networkService: NetworkService
    private let cacheManager: CacheManager
    
    // MARK: - Initialization
    
    /// Initialize the Nigeria Geo client with configuration
    /// - Parameter config: SDK configuration
    public init(config: NigeriaGeoConfig) {
        self.config = config
        self.networkService = NetworkService(config: config)
        self.cacheManager = CacheManager(config: config)
    }
    
    /// Convenience initializer with base URL
    /// - Parameters:
    ///   - baseURL: API base URL
    ///   - apiKey: Optional API key for authentication
    public convenience init(baseURL: String, apiKey: String? = nil) {
        let config = NigeriaGeoConfig(baseURL: baseURL, apiKey: apiKey)
        self.init(config: config)
    }
}

// MARK: - States API

@available(iOS 12.0, macOS 10.15, tvOS 12.0, watchOS 6.0, *)
public extension NigeriaGeoClient {
    
    /// Get all states with pagination
    /// - Parameters:
    ///   - page: Page number (default: 1)
    ///   - limit: Items per page (default: 20)
    ///   - completion: Completion handler
    func getStates(
        page: Int = 1,
        limit: Int = 20,
        completion: @escaping (Result<PaginatedResponse<State>, NigeriaGeoError>) -> Void
    ) {
        let parameters: [String: Any] = [
            "page": page,
            "limit": limit
        ]
        
        let endpoint = APIEndpoint.states(parameters: parameters)
        
        // Check cache first
        if config.enableCaching {
            let cacheKey = endpoint.cacheKey
            if let cachedData: PaginatedResponse<State> = cacheManager.get(key: cacheKey) {
                DispatchQueue.main.async {
                    completion(.success(cachedData))
                }
                return
            }
        }
        
        // Make network request
        networkService.request(endpoint: endpoint) { [weak self] result in
            switch result {
            case .success(let data):
                do {
                    let response = try JSONDecoder().decode(PaginatedResponse<State>.self, from: data)
                    
                    // Cache the response
                    if self?.config.enableCaching == true {
                        self?.cacheManager.set(key: endpoint.cacheKey, value: response)
                    }
                    
                    DispatchQueue.main.async {
                        completion(.success(response))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
    
    /// Get state by ID
    /// - Parameters:
    ///   - id: State UUID
    ///   - completion: Completion handler
    func getState(
        id: UUID,
        completion: @escaping (Result<State?, NigeriaGeoError>) -> Void
    ) {
        let endpoint = APIEndpoint.stateById(id: id)
        
        // Check cache first
        if config.enableCaching {
            let cacheKey = endpoint.cacheKey
            if let cachedData: State? = cacheManager.get(key: cacheKey) {
                DispatchQueue.main.async {
                    completion(.success(cachedData))
                }
                return
            }
        }
        
        networkService.request(endpoint: endpoint) { [weak self] result in
            switch result {
            case .success(let data):
                do {
                    let state = try JSONDecoder().decode(State?.self, from: data)
                    
                    // Cache the response
                    if self?.config.enableCaching == true {
                        self?.cacheManager.set(key: endpoint.cacheKey, value: state)
                    }
                    
                    DispatchQueue.main.async {
                        completion(.success(state))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
    
    /// Get LGAs for a specific state
    /// - Parameters:
    ///   - stateId: State UUID
    ///   - page: Page number (default: 1)
    ///   - limit: Items per page (default: 20)
    ///   - completion: Completion handler
    func getLGAs(
        forState stateId: UUID,
        page: Int = 1,
        limit: Int = 20,
        completion: @escaping (Result<PaginatedResponse<LGA>, NigeriaGeoError>) -> Void
    ) {
        let parameters: [String: Any] = [
            "page": page,
            "limit": limit
        ]
        
        let endpoint = APIEndpoint.lgasByState(stateId: stateId, parameters: parameters)
        
        // Check cache first
        if config.enableCaching {
            let cacheKey = endpoint.cacheKey
            if let cachedData: PaginatedResponse<LGA> = cacheManager.get(key: cacheKey) {
                DispatchQueue.main.async {
                    completion(.success(cachedData))
                }
                return
            }
        }
        
        networkService.request(endpoint: endpoint) { [weak self] result in
            switch result {
            case .success(let data):
                do {
                    let response = try JSONDecoder().decode(PaginatedResponse<LGA>.self, from: data)
                    
                    // Cache the response
                    if self?.config.enableCaching == true {
                        self?.cacheManager.set(key: endpoint.cacheKey, value: response)
                    }
                    
                    DispatchQueue.main.async {
                        completion(.success(response))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
}

// MARK: - Search API

@available(iOS 12.0, macOS 10.15, tvOS 12.0, watchOS 6.0, *)
public extension NigeriaGeoClient {
    
    /// Search across all geographic entities
    /// - Parameters:
    ///   - query: Search query
    ///   - page: Page number (default: 1)
    ///   - limit: Items per page (default: 20)
    ///   - completion: Completion handler
    func searchAll(
        query: String,
        page: Int = 1,
        limit: Int = 20,
        completion: @escaping (Result<SearchResult, NigeriaGeoError>) -> Void
    ) {
        let parameters: [String: Any] = [
            "query": query,
            "page": page,
            "limit": limit
        ]
        
        let endpoint = APIEndpoint.searchAll(parameters: parameters)
        
        networkService.request(endpoint: endpoint) { result in
            switch result {
            case .success(let data):
                do {
                    let response = try JSONDecoder().decode(SearchResult.self, from: data)
                    DispatchQueue.main.async {
                        completion(.success(response))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
    
    /// Find nearby postal codes based on coordinates
    /// - Parameters:
    ///   - latitude: Latitude coordinate
    ///   - longitude: Longitude coordinate
    ///   - radiusKm: Search radius in kilometers (default: 10)
    ///   - completion: Completion handler
    func findNearbyPostalCodes(
        latitude: Double,
        longitude: Double,
        radiusKm: Double = 10.0,
        completion: @escaping (Result<[PostalCode], NigeriaGeoError>) -> Void
    ) {
        let parameters: [String: Any] = [
            "lat": latitude,
            "lng": longitude,
            "radius_km": radiusKm
        ]
        
        let endpoint = APIEndpoint.nearbyPostalCodes(parameters: parameters)
        
        networkService.request(endpoint: endpoint) { result in
            switch result {
            case .success(let data):
                do {
                    let response = try JSONDecoder().decode([PostalCode].self, from: data)
                    DispatchQueue.main.async {
                        completion(.success(response))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
}

// MARK: - Address Validation API

@available(iOS 12.0, macOS 10.15, tvOS 12.0, watchOS 6.0, *)
public extension NigeriaGeoClient {
    
    /// Validate an address
    /// - Parameters:
    ///   - request: Address validation request
    ///   - completion: Completion handler
    func validateAddress(
        _ request: AddressValidationRequest,
        completion: @escaping (Result<AddressValidationResponse, NigeriaGeoError>) -> Void
    ) {
        let endpoint = APIEndpoint.validateAddress
        
        networkService.request(endpoint: endpoint, body: request) { result in
            switch result {
            case .success(let data):
                do {
                    let response = try JSONDecoder().decode(AddressValidationResponse.self, from: data)
                    DispatchQueue.main.async {
                        completion(.success(response))
                    }
                } catch {
                    DispatchQueue.main.async {
                        completion(.failure(.invalidResponse))
                    }
                }
            case .failure(let error):
                DispatchQueue.main.async {
                    completion(.failure(error))
                }
            }
        }
    }
}

// MARK: - Cache Management

@available(iOS 12.0, macOS 10.15, tvOS 12.0, watchOS 6.0, *)
public extension NigeriaGeoClient {
    
    /// Clear all cached data
    /// - Parameter completion: Completion handler
    func clearCache(completion: @escaping (Bool) -> Void) {
        cacheManager.clearAll()
        DispatchQueue.main.async {
            completion(true)
        }
    }
    
    /// Get cache statistics
    /// - Returns: Cache statistics
    func getCacheStatistics() -> CacheStatistics {
        return cacheManager.getStatistics()
    }
}

// MARK: - Async/Await Support (iOS 13+)

@available(iOS 13.0, macOS 10.15, tvOS 13.0, watchOS 6.0, *)
public extension NigeriaGeoClient {
    
    /// Get all states with pagination (async/await)
    func getStates(page: Int = 1, limit: Int = 20) async throws -> PaginatedResponse<State> {
        return try await withCheckedThrowingContinuation { continuation in
            getStates(page: page, limit: limit) { result in
                continuation.resume(with: result)
            }
        }
    }
    
    /// Get state by ID (async/await)
    func getState(id: UUID) async throws -> State? {
        return try await withCheckedThrowingContinuation { continuation in
            getState(id: id) { result in
                continuation.resume(with: result)
            }
        }
    }
    
    /// Search across all geographic entities (async/await)
    func searchAll(query: String, page: Int = 1, limit: Int = 20) async throws -> SearchResult {
        return try await withCheckedThrowingContinuation { continuation in
            searchAll(query: query, page: page, limit: limit) { result in
                continuation.resume(with: result)
            }
        }
    }
    
    /// Validate an address (async/await)
    func validateAddress(_ request: AddressValidationRequest) async throws -> AddressValidationResponse {
        return try await withCheckedThrowingContinuation { continuation in
            validateAddress(request) { result in
                continuation.resume(with: result)
            }
        }
    }
}