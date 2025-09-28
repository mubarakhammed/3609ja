# Nigeria Geo SDK for Flutter

A high-performance Flutter SDK for accessing comprehensive Nigerian geographic data including states, LGAs, wards, and postal codes. Optimized for fast response times with direct database access.

[![Pub Version](https://img.shields.io/pub/v/nigeria_geo_sdk)](https://pub.dev/packages/nigeria_geo_sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🇳🇬 **Complete Nigerian Geographic Data**: All 37 states, 774 LGAs, 8,840+ wards, and postal codes
- ⚡ **High Performance**: Optimized API with < 500ms response times
- 🎯 **Easy Integration**: Simple setup with zero configuration required
- 📱 **Cross Platform**: Works on iOS, Android, Web, and Desktop
- 🔍 **Powerful Search**: Full-text search across all geographic entities
- 📍 **Location Services**: Nearby location finder with GPS integration
- ✅ **Address Validation**: Validate and suggest Nigerian addresses
- 🎨 **UI Flexibility**: Build any interface you want - we provide the data, you design the experience
- 💾 **Smart Caching**: Optional caching for offline support
- 🔒 **Type Safe**: Full Dart type safety with null safety support

## Installation

Add this to your package's `pubspec.yaml` file:

```yaml
dependencies:
  nigeria_geo_sdk: ^1.1.0
```

Then run:

```bash
flutter pub get
```

## Quick Start

### 1. Initialize the SDK

```dart
import 'package:flutter/material.dart';
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  
  // Initialize with default configuration (zero config required!)
  await NigeriaGeoSDK.initialize();
  
  runApp(MyApp());
}
```

### 2. Use the Client

```dart
class MyHomePage extends StatefulWidget {
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final client = NigeriaGeoSDK.client;
  List<NigerianState> states = [];

  @override
  void initState() {
    super.initState();
    loadStates();
  }

  Future<void> loadStates() async {
    try {
      final response = await client.getStates();
      setState(() {
        states = response.data;
      });
    } catch (e) {
      print('Error loading states: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Nigerian States')),
      body: ListView.builder(
        itemCount: states.length,
        itemBuilder: (context, index) {
          final state = states[index];
          return ListTile(
            title: Text(state.name),
            subtitle: Text(state.code),
          );
        },
      ),
    );
  }
}
```

## Configuration

### Default Configuration (Recommended)

The SDK works out of the box with sensible defaults:

```dart
await NigeriaGeoSDK.initialize(); // Uses optimized defaults
```



## Core API Usage

### States API

```dart
// Get all states with pagination
final response = await client.getStates(page: 1, limit: 50);
print('Found ${response.data.length} states');
print('Total: ${response.pagination.total}');

// Get specific state by ID
final state = await client.getStateById('state-id-here');
print('State: ${state.name} (${state.code})');

// Search states
final searchResults = await client.searchStates('Lagos');
```

### LGAs API

```dart
// Get LGAs for a specific state
final lgasResponse = await client.getLGAs(stateId: 'lagos-state-id');

// Get specific LGA
final lga = await client.getLGAById('ikeja-lga-id');

// Search LGAs
final lgaResults = await client.searchLGAs('Ikeja');
```

### Wards API

```dart
// Get wards for a specific LGA
final wardsResponse = await client.getWards(lgaId: 'ikeja-lga-id');

// Get specific ward
final ward = await client.getWardById('ward-id-here');

// Search wards
final wardResults = await client.searchWards('Victoria Island');
```

### Postal Codes API

```dart
// Get postal codes for a ward
final codesResponse = await client.getPostalCodes(wardId: 'ward-id');

// Get by postal code
final location = await client.getPostalCodeByCode('100001');

// Find nearby locations
final nearby = await client.findNearbyPostalCodes(
  latitude: 6.5244,
  longitude: 3.3792,
  radiusKm: 10.0,
);
```

### Search API

```dart
// Search across all entities
final results = await client.searchAll('Lagos');
print('States: ${results.states.length}');
print('LGAs: ${results.lgas.length}');
print('Wards: ${results.wards.length}');
print('Postal codes: ${results.postalCodes.length}');

// Search specific entity types
final states = await client.searchStates('Lagos');
final lgas = await client.searchLGAs('Ikeja');
final wards = await client.searchWards('Victoria');
final postalCodes = await client.searchPostalCodes('100001');
```

### Address Validation

```dart
// Validate an address
final validation = await client.validateAddress(
  AddressValidationRequest(
    state: 'Lagos',
    lga: 'Ikeja',
    ward: 'Ikeja',
    postalCode: '100001',
  ),
);

if (validation.valid) {
  print('Address is valid!');
  print('Canonical: ${validation.canonical}');
} else {
  print('Invalid address. Suggestions:');
  for (final suggestion in validation.suggestions) {
    print('- ${suggestion.reason} (confidence: ${suggestion.confidence})');
  }
}

// Find similar addresses
final similar = await client.findSimilarAddresses(
  AddressValidationRequest(
    state: 'Lagos',
    lga: 'Ikeja',
    ward: 'Ikeja',
    postalCode: '100001',
  ),
);
```

## Building Custom Widgets

**Important**: The SDK provides core API access - you can build any UI you want! The examples below are just samples to get you started.

### Sample: Hierarchical Selection Widgets

Here's an example of how you could build cascading dropdowns using the API:

```dart
class AddressForm extends StatefulWidget {
  @override
  _AddressFormState createState() => _AddressFormState();
}

class _AddressFormState extends State<AddressForm> {
  final client = NigeriaGeoSDK.client;
  NigerianState? selectedState;
  LGA? selectedLGA;
  Ward? selectedWard;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // State picker
        NigeriaStatePicker(
          client: client,
          selectedState: selectedState,
          onStateSelected: (state) {
            setState(() {
              selectedState = state;
              selectedLGA = null; // Reset dependent selections
              selectedWard = null;
            });
          },
        ),
        
        SizedBox(height: 16),
        
        // LGA picker (only enabled when state is selected)
        NigeriaLGAPicker(
          client: client,
          state: selectedState,
          selectedLGA: selectedLGA,
          onLGASelected: (lga) {
            setState(() {
              selectedLGA = lga;
              selectedWard = null; // Reset dependent selection
            });
          },
        ),
        
        SizedBox(height: 16),
        
        // Ward picker (only enabled when LGA is selected)
        NigeriaWardPicker(
          client: client,
          lga: selectedLGA,
          selectedWard: selectedWard,
          onWardSelected: (ward) {
            setState(() {
              selectedWard = ward;
            });
          },
        ),
      ],
    );
  }
}
```

### Sample: Custom Search Widget

You can build any search UI you prefer - here's a simple example:

```dart
class CustomSearchWidget extends StatefulWidget {
  @override
  _CustomSearchWidgetState createState() => _CustomSearchWidgetState();
}

class _CustomSearchWidgetState extends State<CustomSearchWidget> {
  final client = NigeriaGeoSDK.client;
  final _controller = TextEditingController();
  List<dynamic> results = [];

  void _search(String query) async {
    if (query.isEmpty) return;
    
    final searchResults = await client.searchAll(query);
    setState(() {
      results = [
        ...searchResults.states,
        ...searchResults.lgas,
        ...searchResults.wards,
      ];
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        // Build any search UI you want!
        TextField(
          controller: _controller,
          onChanged: _search,
          decoration: InputDecoration(
            hintText: 'Search locations...',
            prefixIcon: Icon(Icons.search),
          ),
        ),
        // Display results however you like
        Expanded(
          child: ListView.builder(
            itemCount: results.length,
            itemBuilder: (context, index) {
              final item = results[index];
              return Card(
                child: ListTile(
                  title: Text(item.name),
                  subtitle: Text(item.runtimeType.toString()),
                  onTap: () {
                    // Handle selection any way you want
                    print('Selected: ${item.name}');
                  },
                ),
              );
            },
          ),
        ),
      ],
    );
  }
}
```

### Sample: Custom Map Integration

```dart
// Build your own map widget with postal codes
class CustomMapWidget extends StatefulWidget {
  @override
  _CustomMapWidgetState createState() => _CustomMapWidgetState();
}

class _CustomMapWidgetState extends State<CustomMapWidget> {
  final client = NigeriaGeoSDK.client;
  List<PostalCode> nearbyLocations = [];

  void _findNearbyLocations(double lat, double lng) async {
    final locations = await client.findNearbyPostalCodes(
      latitude: lat,
      longitude: lng,
      radiusKm: 10.0,
    );
    
    setState(() {
      nearbyLocations = locations;
    });
    
    // Now use these locations with any map package you prefer:
    // Google Maps, Mapbox, OpenStreetMap, etc.
  }

  @override
  Widget build(BuildContext context) {
    // Build your custom map UI here
    return Container(
      child: Text('Your custom map widget with ${nearbyLocations.length} locations'),
    );
  }
}
```

### 🎨 Build Any UI You Want!

The SDK only provides data - you have complete freedom to build:

- **Any dropdown style**: Material, Cupertino, custom designs
- **Any search interface**: Autocomplete, typeahead, instant search
- **Any layout**: Cards, lists, grids, maps, charts
- **Any navigation**: Bottom sheets, dialogs, full-screen pages
- **Any state management**: Provider, Bloc, Riverpod, GetX, etc.

```dart
// Example: Build a completely custom state selector
class FancyStateSelector extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return FutureBuilder<PaginatedResponse<NigerianState>>(
      future: NigeriaGeoSDK.client.getStates(),
      builder: (context, snapshot) {
        if (!snapshot.hasData) return CircularProgressIndicator();
        
        // Build ANY UI you want with the data!
        return GridView.builder(
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(crossAxisCount: 2),
          itemCount: snapshot.data!.data.length,
          itemBuilder: (context, index) {
            final state = snapshot.data!.data[index];
            return Card(
              child: Column(
                children: [
                  Text(state.name, style: TextStyle(fontWeight: FontWeight.bold)),
                  Text(state.code),
                  ElevatedButton(
                    onPressed: () => _selectState(state),
                    child: Text('Select'),
                  ),
                ],
              ),
            );
          },
        );
      },
    );
  }
  
  void _selectState(NigerianState state) {
    // Handle state selection however you want
  }
}
```

## Advanced Usage

### Streams and Reactive Programming

```dart
// Listen to state changes
client.getStatesStream().listen((states) {
  print('States updated: ${states.length}');
});

// Combine with StreamBuilder
StreamBuilder<List<NigerianState>>(
  stream: client.getStatesStream(),
  builder: (context, snapshot) {
    if (snapshot.hasData) {
      return ListView.builder(
        itemCount: snapshot.data!.length,
        itemBuilder: (context, index) {
          final state = snapshot.data![index];
          return ListTile(title: Text(state.name));
        },
      );
    }
    return CircularProgressIndicator();
  },
)
```

### Custom Cache Configuration

```dart
// Initialize with custom caching
await NigeriaGeoSDK.initialize(
  NigeriaGeoConfig(
    baseUrl: 'http://20.63.52.179:3000',
    enableCaching: true,
    cacheExpiration: Duration(hours: 12), // Cache for 12 hours
    maxCacheSize: 50 * 1024 * 1024,      // 50MB cache
  ),
);

// Clear cache when needed
await NigeriaGeoSDK.cacheManager.clearCache();

// Get cache statistics
final stats = await NigeriaGeoSDK.cacheManager.getCacheStats();
print('Cache size: ${stats.size} bytes');
print('Cache entries: ${stats.entries}');
```

### Error Handling

```dart
try {
  final states = await client.getStates();
  // Success
} on NetworkException catch (e) {
  // Network connectivity issues
  print('Network error: ${e.message}');
} on NotFoundException catch (e) {
  // Resource not found
  print('Not found: ${e.message}');
} on RateLimitException catch (e) {
  // Rate limit exceeded
  print('Rate limited: ${e.message}');
} on ServerException catch (e) {
  // Server error
  print('Server error: ${e.message}');
} catch (e) {
  // Other errors
  print('Unexpected error: $e');
}
```

### Pagination Best Practices

```dart
class StatesList extends StatefulWidget {
  @override
  _StatesListState createState() => _StatesListState();
}

class _StatesListState extends State<StatesList> {
  final client = NigeriaGeoSDK.client;
  final List<NigerianState> _states = [];
  int _currentPage = 1;
  bool _isLoading = false;
  bool _hasMore = true;

  @override
  void initState() {
    super.initState();
    _loadMore();
  }

  Future<void> _loadMore() async {
    if (_isLoading || !_hasMore) return;

    setState(() {
      _isLoading = true;
    });

    try {
      final response = await client.getStates(
        page: _currentPage,
        limit: 20,
      );

      setState(() {
        _states.addAll(response.data);
        _currentPage++;
        _hasMore = response.pagination.hasNext;
      });
    } catch (e) {
      print('Error loading more states: $e');
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: _states.length + (_hasMore ? 1 : 0),
      itemBuilder: (context, index) {
        if (index == _states.length) {
          // Load more indicator
          if (!_isLoading) _loadMore();
          return Center(child: CircularProgressIndicator());
        }

        final state = _states[index];
        return ListTile(
          title: Text(state.name),
          subtitle: Text(state.code),
        );
      },
    );
  }
}
```

## Configuration File (Optional)

Create `assets/config.yaml` to override defaults:

```yaml
# Nigeria Geo SDK Configuration
api:
  base_url: "http://20.63.52.179:3000"
  timeout: 5000 # milliseconds

cache:
  enabled: true
  expiration_hours: 6
  max_size_mb: 25

logging:
  enabled: false
  level: "error"

location:
  request_permission: true
  accuracy: "high"
```

Add to `pubspec.yaml`:

```yaml
flutter:
  assets:
    - assets/config.yaml
```

## Performance Tips

1. **Use Pagination**: For large datasets, use pagination instead of fetching all at once
2. **Enable Caching**: Cache responses to reduce API calls and improve offline experience
3. **Debounce Search**: Implement search debouncing to avoid excessive API calls
4. **Preload Data**: Preload commonly used data like states during app initialization

```dart
// Example: Debounced search
class DebouncedSearch extends StatefulWidget {
  @override
  _DebouncedSearchState createState() => _DebouncedSearchState();
}

class _DebouncedSearchState extends State<DebouncedSearch> {
  final _debouncer = Debouncer(milliseconds: 500);
  final client = NigeriaGeoSDK.client;
  List<dynamic> searchResults = [];

  void _onSearchChanged(String query) {
    _debouncer.run(() async {
      if (query.isNotEmpty) {
        final results = await client.searchAll(query);
        setState(() {
          searchResults = [
            ...results.states,
            ...results.lgas,
            ...results.wards,
          ];
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TextField(
          onChanged: _onSearchChanged,
          decoration: InputDecoration(hintText: 'Search...'),
        ),
        Expanded(
          child: ListView.builder(
            itemCount: searchResults.length,
            itemBuilder: (context, index) {
              final result = searchResults[index];
              return ListTile(
                title: Text(result.name),
                subtitle: Text(result.runtimeType.toString()),
              );
            },
          ),
        ),
      ],
    );
  }
}
```

## Testing

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

void main() {
  group('Nigeria Geo SDK Tests', () {
    late NigeriaGeoClient client;

    setUpAll(() async {
      await NigeriaGeoSDK.initialize();
      client = NigeriaGeoSDK.client;
    });

    test('should fetch states', () async {
      final response = await client.getStates();
      expect(response.data, isNotEmpty);
      expect(response.data.first.name, isNotEmpty);
    });

    test('should search locations', () async {
      final results = await client.searchAll('Lagos');
      expect(results.states, isNotEmpty);
    });

    tearDownAll(() async {
      await NigeriaGeoSDK.dispose();
    });
  });
}
```

## API Reference

### Models

- `NigerianState`: Represents a Nigerian state
- `LGA`: Represents a Local Government Area
- `Ward`: Represents a ward within an LGA
- `PostalCode`: Represents a postal code with coordinates
- `PaginatedResponse<T>`: Paginated API response wrapper
- `SearchResult`: Combined search results across all entity types
- `AddressValidationRequest`: Address validation input
- `AddressValidationResponse`: Address validation result

### Client Methods

#### States
- `getStates()`: Get paginated states
- `getStateById(String id)`: Get state by ID
- `searchStates(String query)`: Search states

#### LGAs
- `getLGAs()`: Get paginated LGAs
- `getLGAById(String id)`: Get LGA by ID  
- `searchLGAs(String query)`: Search LGAs

#### Wards
- `getWards()`: Get paginated wards
- `getWardById(String id)`: Get ward by ID
- `searchWards(String query)`: Search wards

#### Postal Codes
- `getPostalCodes()`: Get paginated postal codes
- `getPostalCodeById(String id)`: Get postal code by ID
- `getPostalCodeByCode(String code)`: Get postal code by code value
- `findNearbyPostalCodes()`: Find postal codes near coordinates
- `searchPostalCodes(String query)`: Search postal codes

#### Search & Validation
- `searchAll(String query)`: Search across all entity types
- `validateAddress()`: Validate a Nigerian address
- `findSimilarAddresses()`: Find similar addresses

## Examples

Check out the `example/` directory for a complete Flutter app demonstrating all SDK features:

- API usage examples
- Widget integration
- Search functionality  
- Address validation
- Pagination handling

Run the example:

```bash
cd example
flutter run
```

## Support

- **Documentation**: [GitHub Repository](https://github.com/mubarakhammed/3609ja)
- **Issues**: [Report Issues](https://github.com/mubarakhammed/3609ja/issues)
- **Discussions**: [GitHub Discussions](https://github.com/mubarakhammed/3609ja/discussions)

## Contributing

We welcome contributions! Please see our [Contributing Guide](https://github.com/mubarakhammed/3609ja/blob/main/CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v1.1.0
- Performance optimizations with direct database access  
- Reduced API response times to < 500ms
- Updated configuration defaults for optimal performance
- Enhanced widget pickers to show comprehensive data
- Improved caching strategy

### v1.0.0
- Initial release
- Complete Nigerian geographic data coverage
- Full Flutter integration with widgets
- Comprehensive search and validation features

---

Made with ❤️ for Nigerian developers