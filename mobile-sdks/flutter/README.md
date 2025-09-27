# Nigeria Geo Flutter SDK

Flutter SDK for Nigerian geographic data - states, LGAs, wards, and postal codes.

## Installation

```yaml
dependencies:
  nigeria_geo_sdk: ^1.0.0
```

## Quick Start

```dart
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

void main() async {
  // Initialize SDK (zero configuration required)
  await NigeriaGeoSDK.initialize();
  
  final client = NigeriaGeoSDK.client;
  
  // Get states
  final states = await client.getStates();
  
  // Get LGAs for a state
  final lgas = await client.getLGAs(stateId: states.data.first.id);
  
  // Search across all entities
  final results = await client.searchAll('Lagos');
}
```

## Widgets

Ready-to-use UI components:

```dart
// State picker dropdown
NigeriaStatePicker(
  client: client,
  onStateSelected: (state) => print(state.name),
)

// Hierarchical selection
NigeriaLGAPicker(state: selectedState, ...)
NigeriaWardPicker(lga: selectedLGA, ...)

// Address form
NigeriaAddressForm(
  client: client,
  onAddressChanged: (address) => print(address.formattedAddress),
)

// Search widget
NigeriaGeoSearch(
  client: client,
  onResultSelected: (result) => print(result.title),
)
```

## Features

- **Zero configuration** - works out of the box
- **Complete geographic hierarchy** - States → LGAs → Wards → Postal Codes  
- **Built-in caching** - reduces API calls and improves performance
- **Search functionality** - find any geographic entity
- **UI widgets** - pre-built components for common use cases
- **Address validation** - validate and format Nigerian addresses
- **Cross-platform** - iOS, Android, Web, Desktop

## Example

See the `example/` directory for a complete demo app showcasing all features.

## API Documentation

The SDK provides access to Nigeria's complete geographic data:

- **States**: 36 states + FCT
- **LGAs**: 774 Local Government Areas  
- **Wards**: 8,810+ electoral wards
- **Postal codes**: 1,000+ postal codes with geographic mapping

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize the SDK
  await NigeriaGeoSDK.initialize(
    NigeriaGeoConfig(
      baseUrl: 'https://your-api-base-url.com',
      apiKey: 'your-api-key', // Optional
      enableCaching: true,
      cacheExpiration: Duration(hours: 24),
      enableLogging: true,
    ),
  );
  
  runApp(MyApp());
}
```

### 2. Basic Usage

#### Get All States:

```dart
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

class StatesPage extends StatefulWidget {
  @override
  _StatesPageState createState() => _StatesPageState();
}

class _StatesPageState extends State<StatesPage> {
  final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;
  List<NigerianState> _states = [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadStates();
  }

  Future<void> _loadStates() async {
    try {
      setState(() => _isLoading = true);
      
      final response = await _geoClient.getStates();
      
      setState(() {
        _states = response.data;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    if (_isLoading) {
      return Scaffold(
        appBar: AppBar(title: Text('Nigerian States')),
        body: Center(child: CircularProgressIndicator()),
      );
    }

    if (_error != null) {
      return Scaffold(
        appBar: AppBar(title: Text('Nigerian States')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.error, color: Colors.red, size: 48),
              SizedBox(height: 16),
              Text('Error: $_error'),
              ElevatedButton(
                onPressed: _loadStates,
                child: Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: Text('Nigerian States'),
        actions: [
          IconButton(
            icon: Icon(Icons.refresh),
            onPressed: _loadStates,
          ),
        ],
      ),
      body: ListView.builder(
        itemCount: _states.length,
        itemBuilder: (context, index) {
          final state = _states[index];
          return ListTile(
            title: Text(state.name),
            subtitle: Text('Capital: ${state.capital ?? 'N/A'}'),
            leading: CircleAvatar(
              child: Text(state.code),
            ),
            onTap: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => LGAsPage(stateId: state.id),
                ),
              );
            },
          );
        },
      ),
    );
  }
}
```

### 3. Using Streams

```dart
class StateStreamExample extends StatelessWidget {
  final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('States with Streams')),
      body: StreamBuilder<List<NigerianState>>(
        stream: _geoClient.getStatesStream(),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(child: CircularProgressIndicator());
          }
          
          if (snapshot.hasError) {
            return Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(Icons.error, color: Colors.red),
                  Text('Error: ${snapshot.error}'),
                ],
              ),
            );
          }
          
          final states = snapshot.data ?? [];
          
          return ListView.builder(
            itemCount: states.length,
            itemBuilder: (context, index) {
              return StateListTile(state: states[index]);
            },
          );
        },
      ),
    );
  }
}
```

### 4. Search with Autocomplete

```dart
class LocationSearchDelegate extends SearchDelegate<SearchResultItem?> {
  final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;
  Timer? _debounceTimer;

  @override
  List<Widget> buildActions(BuildContext context) {
    return [
      IconButton(
        icon: Icon(Icons.clear),
        onPressed: () => query = '',
      ),
    ];
  }

  @override
  Widget buildLeading(BuildContext context) {
    return IconButton(
      icon: Icon(Icons.arrow_back),
      onPressed: () => close(context, null),
    );
  }

  @override
  Widget buildResults(BuildContext context) {
    return _buildSearchResults();
  }

  @override
  Widget buildSuggestions(BuildContext context) {
    return _buildSearchResults();
  }

  Widget _buildSearchResults() {
    if (query.length < 2) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(Icons.search, size: 48, color: Colors.grey),
            SizedBox(height: 16),
            Text('Enter at least 2 characters to search'),
          ],
        ),
      );
    }

    return FutureBuilder<SearchResult>(
      future: _performSearch(query),
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return Center(child: CircularProgressIndicator());
        }

        if (snapshot.hasError) {
          return Center(child: Text('Search error: ${snapshot.error}'));
        }

        final result = snapshot.data;
        if (result == null) return Container();

        final allResults = <SearchResultItem>[
          ...result.states.map((s) => SearchResultItem(type: SearchType.state, data: s)),
          ...result.lgas.map((l) => SearchResultItem(type: SearchType.lga, data: l)),
          ...result.wards.map((w) => SearchResultItem(type: SearchType.ward, data: w)),
          ...result.postalCodes.map((p) => SearchResultItem(type: SearchType.postalCode, data: p)),
        ];

        if (allResults.isEmpty) {
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Icons.search_off, size: 48, color: Colors.grey),
                SizedBox(height: 16),
                Text('No results found for "$query"'),
              ],
            ),
          );
        }

        return ListView.builder(
          itemCount: allResults.length,
          itemBuilder: (context, index) {
            final item = allResults[index];
            return SearchResultTile(
              item: item,
              onTap: () => close(context, item),
            );
          },
        );
      },
    );
  }

  Future<SearchResult> _performSearch(String query) async {
    // Debounce search requests
    final completer = Completer<SearchResult>();
    
    _debounceTimer?.cancel();
    _debounceTimer = Timer(Duration(milliseconds: 300), () async {
      try {
        final result = await _geoClient.searchAll(query);
        completer.complete(result);
      } catch (e) {
        completer.completeError(e);
      }
    });
    
    return completer.future;
  }
}
```

## 🎨 UI Components

### StatePicker Widget

```dart
import 'package:nigeria_geo_sdk/widgets.dart';

class AddressFormPage extends StatefulWidget {
  @override
  _AddressFormPageState createState() => _AddressFormPageState();
}

class _AddressFormPageState extends State<AddressFormPage> {
  NigerianState? _selectedState;
  LGA? _selectedLGA;
  Ward? _selectedWard;
  PostalCode? _selectedPostalCode;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Address Form')),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Column(
          children: [
            // State Picker
            StatePicker(
              selectedState: _selectedState,
              onStateSelected: (state) {
                setState(() {
                  _selectedState = state;
                  _selectedLGA = null; // Reset dependent fields
                  _selectedWard = null;
                  _selectedPostalCode = null;
                });
              },
              decoration: InputDecoration(
                labelText: 'Select State',
                border: OutlineInputBorder(),
                prefixIcon: Icon(Icons.location_on),
              ),
            ),
            
            SizedBox(height: 16),
            
            // LGA Picker (enabled only when state is selected)
            LGAPicker(
              selectedState: _selectedState,
              selectedLGA: _selectedLGA,
              onLGASelected: (lga) {
                setState(() {
                  _selectedLGA = lga;
                  _selectedWard = null; // Reset dependent fields
                  _selectedPostalCode = null;
                });
              },
              enabled: _selectedState != null,
              decoration: InputDecoration(
                labelText: 'Select LGA',
                border: OutlineInputBorder(),
                prefixIcon: Icon(Icons.business),
              ),
            ),
            
            SizedBox(height: 16),
            
            // Ward Picker (enabled only when LGA is selected)
            WardPicker(
              selectedLGA: _selectedLGA,
              selectedWard: _selectedWard,
              onWardSelected: (ward) {
                setState(() {
                  _selectedWard = ward;
                  _selectedPostalCode = null; // Reset dependent field
                });
              },
              enabled: _selectedLGA != null,
              decoration: InputDecoration(
                labelText: 'Select Ward',
                border: OutlineInputBorder(),
                prefixIcon: Icon(Icons.domain),
              ),
            ),
            
            SizedBox(height: 16),
            
            // Postal Code Picker (enabled only when ward is selected)
            PostalCodePicker(
              selectedWard: _selectedWard,
              selectedPostalCode: _selectedPostalCode,
              onPostalCodeSelected: (postalCode) {
                setState(() {
                  _selectedPostalCode = postalCode;
                });
              },
              enabled: _selectedWard != null,
              decoration: InputDecoration(
                labelText: 'Select Postal Code',
                border: OutlineInputBorder(),
                prefixIcon: Icon(Icons.mail),
              ),
            ),
            
            SizedBox(height: 24),
            
            // Validate Button
            ElevatedButton(
              onPressed: _canValidate() ? _validateAddress : null,
              child: Text('Validate Address'),
              style: ElevatedButton.styleFrom(
                minimumSize: Size(double.infinity, 48),
              ),
            ),
          ],
        ),
      ),
    );
  }

  bool _canValidate() {
    return _selectedState != null && _selectedLGA != null;
  }

  Future<void> _validateAddress() async {
    if (!_canValidate()) return;

    try {
      final request = AddressValidationRequest(
        state: _selectedState!.name,
        lga: _selectedLGA!.name,
        ward: _selectedWard?.name,
        postalCode: _selectedPostalCode?.code,
      );

      final result = await NigeriaGeoSDK.client.validateAddress(request);

      showDialog(
        context: context,
        builder: (context) => AddressValidationDialog(result: result),
      );
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: Text('Validation error: ${e.toString()}'),
          backgroundColor: Colors.red,
        ),
      );
    }
  }
}
```

### Location Search Widget

```dart
class LocationSearchWidget extends StatefulWidget {
  final Function(SearchResultItem)? onLocationSelected;
  final String hintText;
  final int maxSuggestions;

  const LocationSearchWidget({
    Key? key,
    this.onLocationSelected,
    this.hintText = 'Search for locations...',
    this.maxSuggestions = 10,
  }) : super(key: key);

  @override
  _LocationSearchWidgetState createState() => _LocationSearchWidgetState();
}

class _LocationSearchWidgetState extends State<LocationSearchWidget> {
  final TextEditingController _controller = TextEditingController();
  final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;
  
  List<SearchResultItem> _suggestions = [];
  bool _isLoading = false;
  Timer? _debounceTimer;
  OverlayEntry? _overlayEntry;

  @override
  void initState() {
    super.initState();
    _controller.addListener(_onQueryChanged);
  }

  @override
  void dispose() {
    _controller.removeListener(_onQueryChanged);
    _controller.dispose();
    _debounceTimer?.cancel();
    _hideOverlay();
    super.dispose();
  }

  void _onQueryChanged() {
    final query = _controller.text.trim();
    
    if (query.length < 2) {
      _hideOverlay();
      return;
    }

    _debounceTimer?.cancel();
    _debounceTimer = Timer(Duration(milliseconds: 300), () {
      _performSearch(query);
    });
  }

  Future<void> _performSearch(String query) async {
    setState(() => _isLoading = true);

    try {
      final result = await _geoClient.searchAll(query);
      
      final suggestions = <SearchResultItem>[
        ...result.states.map((s) => SearchResultItem(type: SearchType.state, data: s)),
        ...result.lgas.map((l) => SearchResultItem(type: SearchType.lga, data: l)),
        ...result.wards.map((w) => SearchResultItem(type: SearchType.ward, data: w)),
        ...result.postalCodes.map((p) => SearchResultItem(type: SearchType.postalCode, data: p)),
      ];

      setState(() {
        _suggestions = suggestions.take(widget.maxSuggestions).toList();
        _isLoading = false;
      });

      _showOverlay();
    } catch (e) {
      setState(() => _isLoading = false);
      // Handle error silently or show a brief message
    }
  }

  void _showOverlay() {
    _hideOverlay();

    if (_suggestions.isEmpty) return;

    _overlayEntry = OverlayEntry(
      builder: (context) => Positioned(
        width: MediaQuery.of(context).size.width - 32,
        child: CompositedTransformFollower(
          link: _layerLink,
          showWhenUnlinked: false,
          offset: Offset(0, 56),
          child: Material(
            elevation: 4,
            borderRadius: BorderRadius.circular(8),
            child: Container(
              constraints: BoxConstraints(maxHeight: 300),
              child: ListView.builder(
                shrinkWrap: true,
                itemCount: _suggestions.length,
                itemBuilder: (context, index) {
                  final suggestion = _suggestions[index];
                  return ListTile(
                    leading: Icon(_getIconForType(suggestion.type)),
                    title: Text(suggestion.title),
                    subtitle: Text(suggestion.subtitle),
                    onTap: () {
                      _controller.text = suggestion.title;
                      _hideOverlay();
                      widget.onLocationSelected?.call(suggestion);
                    },
                  );
                },
              ),
            ),
          ),
        ),
      ),
    );

    Overlay.of(context)?.insert(_overlayEntry!);
  }

  void _hideOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
  }

  final LayerLink _layerLink = LayerLink();

  @override
  Widget build(BuildContext context) {
    return CompositedTransformTarget(
      link: _layerLink,
      child: TextField(
        controller: _controller,
        decoration: InputDecoration(
          hintText: widget.hintText,
          prefixIcon: Icon(Icons.search),
          suffixIcon: _isLoading
              ? Container(
                  width: 20,
                  height: 20,
                  padding: EdgeInsets.all(12),
                  child: CircularProgressIndicator(strokeWidth: 2),
                )
              : _controller.text.isNotEmpty
                  ? IconButton(
                      icon: Icon(Icons.clear),
                      onPressed: () {
                        _controller.clear();
                        _hideOverlay();
                      },
                    )
                  : null,
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
          ),
        ),
      ),
    );
  }

  IconData _getIconForType(SearchType type) {
    switch (type) {
      case SearchType.state:
        return Icons.location_on;
      case SearchType.lga:
        return Icons.business;
      case SearchType.ward:
        return Icons.domain;
      case SearchType.postalCode:
        return Icons.mail;
    }
  }
}
```

## 📍 Location Services Integration

```dart
import 'package:geolocator/geolocator.dart';
import 'package:permission_handler/permission_handler.dart';

class LocationService {
  static final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;

  /// Get current location and nearby postal codes
  static Future<LocationContext> getCurrentLocationContext() async {
    // Check and request location permission
    final permission = await Permission.location.status;
    if (permission.isDenied) {
      final result = await Permission.location.request();
      if (result.isDenied) {
        throw LocationException('Location permission denied');
      }
    }

    // Get current position
    final position = await Geolocator.getCurrentPosition(
      desiredAccuracy: LocationAccuracy.high,
    );

    // Find nearby postal codes
    final nearbyPostalCodes = await _geoClient.findNearbyPostalCodes(
      latitude: position.latitude,
      longitude: position.longitude,
      radiusKm: 5.0,
    );

    return LocationContext(
      latitude: position.latitude,
      longitude: position.longitude,
      nearbyPostalCodes: nearbyPostalCodes,
    );
  }

  /// Stream location updates
  static Stream<LocationContext> getLocationStream() async* {
    final locationSettings = LocationSettings(
      accuracy: LocationAccuracy.high,
      distanceFilter: 100, // Update every 100 meters
    );

    await for (final position in Geolocator.getPositionStream(settings: locationSettings)) {
      try {
        final nearbyPostalCodes = await _geoClient.findNearbyPostalCodes(
          latitude: position.latitude,
          longitude: position.longitude,
          radiusKm: 5.0,
        );

        yield LocationContext(
          latitude: position.latitude,
          longitude: position.longitude,
          nearbyPostalCodes: nearbyPostalCodes,
        );
      } catch (e) {
        // Handle error silently or emit error state
      }
    }
  }
}

class LocationContext {
  final double latitude;
  final double longitude;
  final List<PostalCode> nearbyPostalCodes;

  LocationContext({
    required this.latitude,
    required this.longitude,
    required this.nearbyPostalCodes,
  });
}

class LocationException implements Exception {
  final String message;
  LocationException(this.message);

  @override
  String toString() => 'LocationException: $message';
}
```

## 💾 Caching and Offline Support

### Automatic Caching

```dart
// Caching is handled automatically based on configuration
final config = NigeriaGeoConfig(
  baseUrl: 'https://api.example.com',
  enableCaching: true,
  cacheExpiration: Duration(hours: 24),
  maxCacheSize: 50 * 1024 * 1024, // 50MB
);

await NigeriaGeoSDK.initialize(config);
```

### Manual Cache Management

```dart
final cacheManager = NigeriaGeoSDK.cacheManager;

// Clear all cache
await cacheManager.clearAll();

// Clear specific entity cache
await cacheManager.clearStatesCache();
await cacheManager.clearSearchCache();

// Get cache statistics
final stats = await cacheManager.getStatistics();
print('Cache size: ${stats.sizeInMB.toStringAsFixed(2)} MB');
print('Hit rate: ${stats.hitRate.toStringAsFixed(1)}%');
print('Total items: ${stats.itemCount}');

// Check if data exists in cache
final hasStatesCache = await cacheManager.hasStatesCache();
if (!hasStatesCache) {
  // Preload states for offline use
  await NigeriaGeoSDK.client.getStates();
}
```

## 🛡️ Error Handling

### Exception Types

```dart
abstract class NigeriaGeoException implements Exception {
  final String message;
  final String? code;
  
  const NigeriaGeoException(this.message, [this.code]);
  
  @override
  String toString() => 'NigeriaGeoException: $message${code != null ? ' (Code: $code)' : ''}';
}

class NetworkException extends NigeriaGeoException {
  NetworkException(String message) : super(message, 'NETWORK_ERROR');
}

class InvalidResponseException extends NigeriaGeoException {
  InvalidResponseException(String message) : super(message, 'INVALID_RESPONSE');
}

class UnauthorizedException extends NigeriaGeoException {
  UnauthorizedException(String message) : super(message, 'UNAUTHORIZED');
}

class NotFoundException extends NigeriaGeoException {
  NotFoundException(String message) : super(message, 'NOT_FOUND');
}

class RateLimitException extends NigeriaGeoException {
  RateLimitException(String message) : super(message, 'RATE_LIMIT');
}

class CacheException extends NigeriaGeoException {
  CacheException(String message) : super(message, 'CACHE_ERROR');
}
```

### Error Recovery

```dart
class ErrorHandlingExample extends StatefulWidget {
  @override
  _ErrorHandlingExampleState createState() => _ErrorHandlingExampleState();
}

class _ErrorHandlingExampleState extends State<ErrorHandlingExample> {
  final NigeriaGeoClient _geoClient = NigeriaGeoSDK.client;
  
  Future<void> _loadStatesWithRetry() async {
    int retryCount = 0;
    const maxRetries = 3;
    
    while (retryCount < maxRetries) {
      try {
        final states = await _geoClient.getStates();
        // Handle success
        _handleStatesLoaded(states.data);
        return;
      } catch (e) {
        retryCount++;
        
        if (e is NetworkException) {
          // Try loading from cache
          final cachedStates = await _tryLoadFromCache();
          if (cachedStates != null) {
            _handleStatesLoaded(cachedStates);
            return;
          }
          
          // Retry with exponential backoff
          if (retryCount < maxRetries) {
            await Future.delayed(Duration(seconds: math.pow(2, retryCount).toInt()));
            continue;
          }
        } else if (e is RateLimitException) {
          // Show rate limit message
          _showRateLimitDialog();
          return;
        } else if (e is UnauthorizedException) {
          // Show authentication error
          _showAuthenticationError();
          return;
        }
        
        // Final retry failed
        if (retryCount >= maxRetries) {
          _showGenericError(e.toString());
        }
      }
    }
  }
  
  Future<List<NigerianState>?> _tryLoadFromCache() async {
    try {
      final cacheManager = NigeriaGeoSDK.cacheManager;
      return await cacheManager.getCachedStates();
    } catch (e) {
      return null;
    }
  }
  
  void _handleStatesLoaded(List<NigerianState> states) {
    // Update UI with states
    setState(() {
      // Update state
    });
  }
  
  void _showRateLimitDialog() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Rate Limit Exceeded'),
        content: Text('Too many requests. Please try again later.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text('OK'),
          ),
        ],
      ),
    );
  }
  
  void _showAuthenticationError() {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text('Authentication Error'),
        content: Text('Invalid API key. Please check your configuration.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: Text('OK'),
          ),
        ],
      ),
    );
  }
  
  void _showGenericError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text('Error: $message'),
        backgroundColor: Colors.red,
        action: SnackBarAction(
          label: 'Retry',
          onPressed: _loadStatesWithRetry,
        ),
      ),
    );
  }
  
  // ... rest of widget implementation
}
```

## 🧪 Testing

### Unit Tests

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

// Generate mocks
@GenerateMocks([NigeriaGeoClient])
void main() {
  group('NigeriaGeoClient Tests', () {
    late MockNigeriaGeoClient mockClient;
    
    setUp(() {
      mockClient = MockNigeriaGeoClient();
    });
    
    test('getStates returns list of states', () async {
      // Arrange
      final mockResponse = PaginatedResponse<NigerianState>(
        data: [
          NigerianState(
            id: '1',
            name: 'Lagos',
            code: 'LA',
            capital: 'Ikeja',
          ),
        ],
        pagination: PaginationMeta(
          page: 1,
          limit: 20,
          total: 37,
          totalPages: 2,
          hasNext: true,
          hasPrev: false,
        ),
      );
      
      when(mockClient.getStates()).thenAnswer((_) async => mockResponse);
      
      // Act
      final result = await mockClient.getStates();
      
      // Assert
      expect(result.data.length, 1);
      expect(result.data.first.name, 'Lagos');
      expect(result.pagination.total, 37);
      
      verify(mockClient.getStates()).called(1);
    });
    
    test('searchAll returns search results', () async {
      // Arrange
      final mockResult = SearchResult(
        states: [
          NigerianState(id: '1', name: 'Lagos', code: 'LA'),
        ],
        lgas: [],
        wards: [],
        postalCodes: [],
      );
      
      when(mockClient.searchAll('lagos')).thenAnswer((_) async => mockResult);
      
      // Act
      final result = await mockClient.searchAll('lagos');
      
      // Assert
      expect(result.states.length, 1);
      expect(result.states.first.name, 'Lagos');
      
      verify(mockClient.searchAll('lagos')).called(1);
    });
    
    test('validateAddress returns validation result', () async {
      // Arrange
      final request = AddressValidationRequest(
        state: 'Lagos',
        lga: 'Ikeja',
      );
      
      final mockResponse = AddressValidationResponse(
        isValid: true,
        confidence: 0.95,
        normalizedAddress: NormalizedAddress(
          state: 'Lagos',
          lga: 'Ikeja',
        ),
        suggestions: [],
        errors: [],
      );
      
      when(mockClient.validateAddress(request))
          .thenAnswer((_) async => mockResponse);
      
      // Act
      final result = await mockClient.validateAddress(request);
      
      // Assert
      expect(result.isValid, true);
      expect(result.confidence, 0.95);
      
      verify(mockClient.validateAddress(request)).called(1);
    });
  });
}
```

### Widget Tests

```dart
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mockito/mockito.dart';
import 'package:nigeria_geo_sdk/widgets.dart';

void main() {
  group('StatePicker Widget Tests', () {
    testWidgets('displays loading state initially', (tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: StatePicker(
              onStateSelected: (state) {},
            ),
          ),
        ),
      );
      
      expect(find.byType(CircularProgressIndicator), findsOneWidget);
    });
    
    testWidgets('displays states after loading', (tester) async {
      // Mock the client to return test data
      // ... setup mock
      
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: StatePicker(
              onStateSelected: (state) {},
            ),
          ),
        ),
      );
      
      await tester.pumpAndSettle();
      
      expect(find.text('Lagos'), findsOneWidget);
      expect(find.text('Abuja'), findsOneWidget);
    });
    
    testWidgets('calls onStateSelected when state is tapped', (tester) async {
      NigerianState? selectedState;
      
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: StatePicker(
              onStateSelected: (state) {
                selectedState = state;
              },
            ),
          ),
        ),
      );
      
      await tester.pumpAndSettle();
      
      await tester.tap(find.text('Lagos'));
      
      expect(selectedState?.name, 'Lagos');
    });
  });
}
```

### Integration Tests

```dart
import 'package:flutter_driver/flutter_driver.dart';
import 'package:test/test.dart';

void main() {
  group('Nigeria Geo SDK Integration Tests', () {
    FlutterDriver? driver;
    
    setUpAll(() async {
      driver = await FlutterDriver.connect();
    });
    
    tearDownAll(() async {
      await driver?.close();
    });
    
    test('complete address selection flow', () async {
      // Navigate to address form
      await driver!.tap(find.byValueKey('address_form_button'));
      
      // Select state
      await driver!.tap(find.byValueKey('state_picker'));
      await driver!.tap(find.text('Lagos'));
      
      // Select LGA
      await driver!.tap(find.byValueKey('lga_picker'));
      await driver!.tap(find.text('Ikeja'));
      
      // Verify validation button is enabled
      expect(
        await driver!.getText(find.byValueKey('validate_button')),
        'Validate Address',
      );
      
      // Tap validate button
      await driver!.tap(find.byValueKey('validate_button'));
      
      // Wait for validation result
      await driver!.waitFor(find.byValueKey('validation_result'));
      
      // Verify result is shown
      expect(
        await driver!.getText(find.byValueKey('validation_status')),
        contains('Valid'),
      );
    });
  });
}
```

## 📚 Example Projects

Check out our example applications:

- **basic_example** - Simple usage examples and setup
- **complete_app** - Full-featured app with all SDK capabilities  
- **location_tracker** - Location services and nearby searches
- **address_book** - Contact management with address validation
- **delivery_app** - Delivery application with route optimization

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## 📞 Support

- **📧 Email**: flutter-sdk@nigeria-geo.com
- **🐛 Issues**: [GitHub Issues](../../issues)
- **💬 Discussions**: [GitHub Discussions](../../discussions)  
- **📖 Documentation**: [Full Documentation](https://docs.nigeria-geo.com/flutter)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](../../LICENSE) file for details.