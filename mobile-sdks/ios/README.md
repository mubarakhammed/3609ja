# 📱 Nigeria Geo iOS SDK

Native Swift SDK for integrating Nigerian geographic data into iOS applications.

## 🚀 Features

- ✅ Native Swift implementation
- 🗺️ Complete geographic hierarchy (States → LGAs → Wards → Postal Codes)
- 📍 Location services integration
- 💾 Intelligent caching with Core Data
- 🔍 Smart search with autocomplete
- ✅ Real-time address validation
- 📱 Pre-built UI components
- 🚀 Async/await support (iOS 13+)
- 🛡️ Comprehensive error handling

## 📋 Requirements

- iOS 12.0+
- Xcode 14.0+
- Swift 5.7+

## 🛠️ Installation

### Swift Package Manager (Recommended)

Add to your `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/mubarakhammed/nigeria-geo-ios-sdk.git", from: "1.0.0")
]
```

Or in Xcode:
1. File → Add Package Dependencies
2. Enter: `https://github.com/mubarakhammed/nigeria-geo-ios-sdk.git`
3. Select version and add to target

### CocoaPods

Add to your `Podfile`:

```ruby
pod 'NigeriaGeoSDK', '~> 1.0'
```

Then run:
```bash
pod install
```

## 🎯 Quick Start

### 1. Import and Configure

```swift
import NigeriaGeoSDK

// Configure the SDK
let config = NigeriaGeoConfig(
    baseURL: "https://your-api-base-url.com",
    apiKey: "your-api-key", // Optional
    enableCaching: true,
    cacheExpiration: .hours(24),
    requestTimeout: 30.0
)

// Initialize the client
let geoClient = NigeriaGeoClient(config: config)
```

### 2. Basic Operations

#### Get All States
```swift
// Using completion handlers
geoClient.getStates { result in
    switch result {
    case .success(let response):
        let states = response.data
        print("Found \\(states.count) states")
        // Update UI
    case .failure(let error):
        print("Error: \\(error.localizedDescription)")
    }
}

// Using async/await (iOS 13+)
Task {
    do {
        let response = try await geoClient.getStates()
        print("Found \\(response.data.count) states")
    } catch {
        print("Error: \\(error)")
    }
}
```

#### Get State by ID
```swift
let stateId = UUID(uuidString: "550e8400-e29b-41d4-a716-446655440001")!

geoClient.getState(id: stateId) { result in
    switch result {
    case .success(let state):
        if let state = state {
            print("State: \\(state.name)")
        } else {
            print("State not found")
        }
    case .failure(let error):
        print("Error: \\(error)")
    }
}
```

#### Search Locations
```swift
geoClient.searchAll(query: "lagos") { result in
    switch result {
    case .success(let searchResult):
        print("States: \\(searchResult.states.count)")
        print("LGAs: \\(searchResult.lgas.count)")
        print("Wards: \\(searchResult.wards.count)")
    case .failure(let error):
        print("Search error: \\(error)")
    }
}
```

### 3. Location Services Integration

```swift
import CoreLocation

class LocationManager: NSObject, CLLocationManagerDelegate {
    private let geoClient: NigeriaGeoClient
    private let locationManager = CLLocationManager()
    
    init(geoClient: NigeriaGeoClient) {
        self.geoClient = geoClient
        super.init()
        setupLocationManager()
    }
    
    private func setupLocationManager() {
        locationManager.delegate = self
        locationManager.desiredAccuracy = kCLLocationAccuracyBest
        locationManager.requestWhenInUseAuthorization()
    }
    
    func getCurrentGeographicContext() {
        locationManager.requestLocation()
    }
    
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        guard let location = locations.last else { return }
        
        // Find nearby postal codes
        geoClient.findNearbyPostalCodes(
            latitude: location.coordinate.latitude,
            longitude: location.coordinate.longitude,
            radiusKm: 5.0
        ) { result in
            switch result {
            case .success(let postalCodes):
                print("Found \\(postalCodes.count) nearby postal codes")
            case .failure(let error):
                print("Location search error: \\(error)")
            }
        }
    }
}
```

### 4. Address Validation

```swift
let address = AddressValidationRequest(
    state: "Lagos",
    lga: "Ikeja",
    ward: "Ikeja",
    postalCode: "100001"
)

geoClient.validateAddress(address) { result in
    switch result {
    case .success(let validation):
        if validation.isValid {
            print("Address is valid!")
            print("Confidence: \\(validation.confidence)")
        } else {
            print("Invalid address")
            print("Suggestions: \\(validation.suggestions)")
        }
    case .failure(let error):
        print("Validation error: \\(error)")
    }
}
```

## 🎨 UI Components

### StatePickerViewController

A ready-to-use view controller for state selection:

```swift
import NigeriaGeoSDK

class MyViewController: UIViewController {
    
    @IBAction func selectStateButtonTapped(_ sender: UIButton) {
        let statePicker = StatePickerViewController(geoClient: geoClient)
        
        statePicker.onStateSelected = { [weak self] state in
            print("Selected state: \\(state.name)")
            self?.dismiss(animated: true)
        }
        
        statePicker.onCancelled = { [weak self] in
            self?.dismiss(animated: true)
        }
        
        let navController = UINavigationController(rootViewController: statePicker)
        present(navController, animated: true)
    }
}
```

### AddressFormView

A SwiftUI view for address input:

```swift
import SwiftUI
import NigeriaGeoSDK

struct ContentView: View {
    @StateObject private var addressForm = AddressFormViewModel()
    
    var body: some View {
        NavigationView {
            AddressFormView(
                viewModel: addressForm,
                geoClient: geoClient
            )
            .navigationTitle("Enter Address")
        }
    }
}

struct AddressFormView: View {
    @ObservedObject var viewModel: AddressFormViewModel
    let geoClient: NigeriaGeoClient
    
    var body: some View {
        Form {
            Section("Geographic Information") {
                StatePicker(
                    selectedState: $viewModel.selectedState,
                    geoClient: geoClient
                )
                
                if let state = viewModel.selectedState {
                    LGAPicker(
                        selectedLGA: $viewModel.selectedLGA,
                        state: state,
                        geoClient: geoClient
                    )
                }
                
                if let lga = viewModel.selectedLGA {
                    WardPicker(
                        selectedWard: $viewModel.selectedWard,
                        lga: lga,
                        geoClient: geoClient
                    )
                }
            }
            
            Section("Postal Information") {
                PostalCodeField(
                    postalCode: $viewModel.postalCode,
                    geoClient: geoClient
                )
            }
            
            Section("Validation") {
                ValidationResultView(
                    validation: viewModel.validation
                )
            }
        }
    }
}
```

### LocationSearchBar

A search bar with autocomplete:

```swift
import UIKit
import NigeriaGeoSDK

class LocationSearchViewController: UIViewController {
    @IBOutlet weak var searchBar: UISearchBar!
    @IBOutlet weak var tableView: UITableView!
    
    private let geoClient: NigeriaGeoClient
    private var searchResults: [SearchResultItem] = []
    
    init(geoClient: NigeriaGeoClient) {
        self.geoClient = geoClient
        super.init(nibName: nil, bundle: nil)
    }
    
    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        setupUI()
    }
    
    private func setupUI() {
        searchBar.delegate = self
        tableView.delegate = self
        tableView.dataSource = self
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "Cell")
    }
}

extension LocationSearchViewController: UISearchBarDelegate {
    func searchBar(_ searchBar: UISearchBar, textDidChange searchText: String) {
        guard searchText.count >= 2 else {
            searchResults = []
            tableView.reloadData()
            return
        }
        
        geoClient.searchAll(query: searchText) { [weak self] result in
            DispatchQueue.main.async {
                switch result {
                case .success(let result):
                    self?.searchResults = self?.flattenSearchResults(result) ?? []
                    self?.tableView.reloadData()
                case .failure(let error):
                    print("Search error: \\(error)")
                }
            }
        }
    }
    
    private func flattenSearchResults(_ result: SearchResult) -> [SearchResultItem] {
        var items: [SearchResultItem] = []
        
        items.append(contentsOf: result.states.map { SearchResultItem(type: .state, data: $0) })
        items.append(contentsOf: result.lgas.map { SearchResultItem(type: .lga, data: $0) })
        items.append(contentsOf: result.wards.map { SearchResultItem(type: .ward, data: $0) })
        items.append(contentsOf: result.postalCodes.map { SearchResultItem(type: .postalCode, data: $0) })
        
        return items
    }
}
```

## 💾 Caching

The SDK includes intelligent caching to improve performance and enable offline usage:

### Configuration
```swift
let config = NigeriaGeoConfig(
    baseURL: "https://api.example.com",
    enableCaching: true,
    cacheExpiration: .hours(24), // Cache duration
    maxCacheSize: .megabytes(50) // Maximum cache size
)
```

### Manual Cache Management
```swift
// Clear all cached data
geoClient.clearCache { success in
    print("Cache cleared: \\(success)")
}

// Get cache statistics
let cacheStats = geoClient.getCacheStatistics()
print("Cache size: \\(cacheStats.sizeInBytes) bytes")
print("Cached items: \\(cacheStats.itemCount)")
```

## 🛡️ Error Handling

The SDK provides comprehensive error handling:

### Error Types
```swift
enum NigeriaGeoError: Error, LocalizedError {
    case networkError(URLError)
    case invalidResponse
    case invalidURL
    case unauthorized
    case notFound
    case serverError(Int)
    case rateLimitExceeded
    case cacheError(String)
    
    var errorDescription: String? {
        switch self {
        case .networkError(let urlError):
            return "Network error: \\(urlError.localizedDescription)"
        case .invalidResponse:
            return "Invalid response format"
        case .invalidURL:
            return "Invalid URL"
        case .unauthorized:
            return "Unauthorized access - check API key"
        case .notFound:
            return "Resource not found"
        case .serverError(let code):
            return "Server error: \\(code)"
        case .rateLimitExceeded:
            return "Rate limit exceeded - try again later"
        case .cacheError(let message):
            return "Cache error: \\(message)"
        }
    }
}
```

### Error Recovery
```swift
geoClient.getStates { result in
    switch result {
    case .success(let response):
        // Handle success
        break
    case .failure(let error):
        if let geoError = error as? NigeriaGeoError {
            switch geoError {
            case .networkError:
                // Try to load from cache or show offline message
                self.loadStatesFromCache()
            case .rateLimitExceeded:
                // Show rate limit message and retry after delay
                self.scheduleRetry()
            case .unauthorized:
                // Prompt user to check API configuration
                self.showAPIKeyError()
            default:
                // Show generic error message
                self.showError(geoError.localizedDescription)
            }
        }
    }
}
```

## 🧪 Testing

### Unit Tests
```swift
import XCTest
@testable import NigeriaGeoSDK

class NigeriaGeoClientTests: XCTestCase {
    private var geoClient: NigeriaGeoClient!
    private var mockSession: URLSession!
    
    override func setUp() {
        super.setUp()
        let config = NigeriaGeoConfig(
            baseURL: "https://test-api.example.com",
            enableCaching: false // Disable caching for tests
        )
        geoClient = NigeriaGeoClient(config: config)
    }
    
    func testGetStates() {
        let expectation = XCTestExpectation(description: "Get states")
        
        geoClient.getStates { result in
            switch result {
            case .success(let response):
                XCTAssertFalse(response.data.isEmpty)
                XCTAssertEqual(response.pagination.total, 37) // Expected number of states
            case .failure(let error):
                XCTFail("Expected success, got error: \\(error)")
            }
            expectation.fulfill()
        }
        
        wait(for: [expectation], timeout: 10.0)
    }
    
    func testStateSearch() {
        let expectation = XCTestExpectation(description: "Search states")
        
        geoClient.searchAll(query: "lagos") { result in
            switch result {
            case .success(let searchResult):
                XCTAssertTrue(searchResult.states.contains { $0.name.lowercased().contains("lagos") })
            case .failure(let error):
                XCTFail("Search failed: \\(error)")
            }
            expectation.fulfill()
        }
        
        wait(for: [expectation], timeout: 10.0)
    }
}
```

## 📊 Performance Tips

### Optimization Strategies
1. **Use Pagination**: Don't load all data at once
2. **Enable Caching**: Reduces network requests
3. **Batch Requests**: Combine multiple operations
4. **Background Processing**: Use background queues for API calls
5. **Image Optimization**: Use appropriate image sizes

### Performance Monitoring
```swift
// Enable performance monitoring
let config = NigeriaGeoConfig(
    baseURL: "https://api.example.com",
    enablePerformanceMonitoring: true
)

// Access performance metrics
let metrics = geoClient.getPerformanceMetrics()
print("Average response time: \\(metrics.averageResponseTime)ms")
print("Cache hit rate: \\(metrics.cacheHitRate)%")
```

## 📚 Examples

Check out our example projects:

- **BasicIntegration** - Simple implementation examples
- **AdvancedFeatures** - Complex use cases and optimizations
- **UIKitIntegration** - Complete UIKit application
- **SwiftUIIntegration** - Modern SwiftUI application

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## 📞 Support

- **📧 Email**: ios-sdk@nigeria-geo.com
- **🐛 Issues**: [GitHub Issues](../../issues)
- **💬 Discussions**: [GitHub Discussions](../../discussions)
- **📖 Documentation**: [Full Documentation](https://docs.nigeria-geo.com/ios)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.