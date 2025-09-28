# Nigeria Geo SDK for React Native# Nigeria Geo React Native SDK



A high-performance React Native SDK for accessing comprehensive Nigerian geographic data including states, LGAs, wards, and postal codes. Optimized for fast response times with direct database access.A React Native SDK for accessing Nigerian geographic data including states, LGAs, wards, and postal codes.



[![npm version](https://img.shields.io/npm/v/nigeria-geo-sdk)](https://www.npmjs.com/package/nigeria-geo-sdk)## Installation

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

```bash

## Featuresnpm install @nigeriageo/react-native-sdk

# or

- 🇳🇬 **Complete Nigerian Geographic Data**: All 37 states, 774 LGAs, 8,840+ wards, and postal codesyarn add @nigeriageo/react-native-sdk

- ⚡ **High Performance**: Optimized API with < 500ms response times```

- 📱 **Cross Platform**: Works with React Native, Expo, and bare React Native projects

- 🎯 **Zero Configuration**: Works out of the box with sensible defaults## Configuration

- 🔍 **Powerful Search**: Full-text search across all geographic entities

- 📍 **Location Services**: Geographic data with postal codes### Using Config File

- 🎨 **TypeScript Support**: Full TypeScript definitions included

- 💾 **Smart Caching**: Built-in caching for better performance1. Copy the example config file:

- 🔒 **Type Safe**: Complete TypeScript support with strong typing```bash

cp assets/config.yaml.example assets/config.yaml

## Installation```



### npm2. Update the configuration values in `assets/config.yaml`:

```bash```yaml

npm install nigeria-geo-sdkdevelopment:

```  baseUrl: "http://localhost:3000"

  timeout: 30000

### yarn  enableCaching: true

```bash  enableLogging: true

yarn add nigeria-geo-sdk

```production:

  baseUrl: "https://your-production-api.com"

### Expo  timeout: 15000

```bash  enableCaching: true

expo install nigeria-geo-sdk  enableLogging: false

``````



## Quick Start### Using Environment Variables



### Basic Usage (Zero Configuration)Set `NODE_ENV` to automatically select the appropriate configuration:

```bash

```typescriptNODE_ENV=production # Uses production config

import nigeriaGeoSDK from 'nigeria-geo-sdk';NODE_ENV=staging    # Uses staging config  

NODE_ENV=development # Uses development config (default)

// Get all Nigerian states```

const states = await nigeriaGeoSDK.getStates();

console.log('States:', states);### Programmatic Configuration



// Get LGAs in a state```typescript

const lgas = await nigeriaGeoSDK.getLgasByState(states[0].id);import { NigeriaGeoSDK } from '@nigeriageo/react-native-sdk';

console.log('LGAs:', lgas);

// Zero-config usage (uses development defaults)

// Search for locationsconst sdk = NigeriaGeoSDK.getInstance();

const searchResults = await nigeriaGeoSDK.search('Lagos');

console.log('Search results:', searchResults);// With custom config

```const sdk = NigeriaGeoSDK.getInstance({

  baseUrl: 'https://api.example.com',

### Custom Configuration  timeout: 20000,

  enableLogging: false

```typescript});

import { NigeriaGeoSDK } from 'nigeria-geo-sdk';

// With environment specification

// Initialize with custom configconst sdk = NigeriaGeoSDK.getInstance({}, 'production');

const sdk = NigeriaGeoSDK.getInstance({

  baseUrl: 'http://20.63.52.179:3000', // Production server// Reconfigure existing instance

  timeout: 5000,                        // 5 second timeoutawait NigeriaGeoSDK.reconfigure({

  enableCaching: true,                  // Enable caching  baseUrl: 'https://new-api.example.com'

  enableLogging: false,                 // Disable logging}, 'staging');

});```



// Use the configured SDK## Usage

const states = await sdk.getStates();

``````typescript

import nigeriaGeoSDK from '@nigeriageo/react-native-sdk';

## API Reference

// Get all states

### Configuration Optionsconst states = await nigeriaGeoSDK.getStates();



```typescript// Get LGAs for a state

interface Config {const lgas = await nigeriaGeoSDK.getLgasByState(stateId);

  baseUrl: string;        // API server URL

  timeout: number;        // Request timeout in milliseconds// Get wards for an LGA

  enableCaching: boolean; // Enable response cachingconst wards = await nigeriaGeoSDK.getWardsByLga(lgaId);

  enableLogging: boolean; // Enable debug logging

}// Search functionality

```const results = await nigeriaGeoSDK.search('Lagos');

```

### Data Types

## Configuration Options

```typescript

interface State {| Option | Type | Default | Description |

  id: string;|--------|------|---------|-------------|

  name: string;| `baseUrl` | string | `http://localhost:3000` | API base URL |

  code: string;| `timeout` | number | `30000` | Request timeout in milliseconds |

  created_at: string;| `enableCaching` | boolean | `true` | Enable response caching |

  updated_at: string;| `enableLogging` | boolean | `true` | Enable error logging |

}

## Environment-Specific Defaults

interface Lga {

  id: string;- **Development**: Full logging, localhost API, long timeout

  name: string;- **Staging**: Logging enabled, staging API, medium timeout  

  code: string;- **Production**: Logging disabled, production API, short timeout

  state_id: string;

  created_at: string;## Security Note

  updated_at: string;

}Never commit your actual `config.yaml` file with production API keys or sensitive URLs. Always use the example file as a template and configure your actual values locally or through environment variables.

interface Ward {
  id: string;
  name: string;
  code: string;
  lga_id: string;
  created_at: string;
  updated_at: string;
}

interface PostalCode {
  id: string;
  code: string;
  state_id: string;
  created_at: string;
  updated_at: string;
}

interface SearchResult {
  states: State[];
  lgas: Lga[];
  wards: Ward[];
  postal_codes: PostalCode[];
}
```

### Methods

#### States API

```typescript
// Get all states (with optional limit)
const states = await sdk.getStates(50); // Default: 50 states

// Get specific state by ID
const state = await sdk.getState('state-id-here');
```

#### LGAs API

```typescript
// Get LGAs by state (with optional limit)
const lgas = await sdk.getLgasByState('state-id', 50); // Default: 50 LGAs

// Get specific LGA by ID
const lga = await sdk.getLga('lga-id-here');
```

#### Wards API

```typescript
// Get wards by LGA (with optional limit)
const wards = await sdk.getWardsByLga('lga-id', 100); // Default: 100 wards

// Get specific ward by ID
const ward = await sdk.getWard('ward-id-here');
```

#### Postal Codes API

```typescript
// Get postal codes by state
const postalCodes = await sdk.getPostalCodesByState('state-id');

// Get specific postal code by ID
const postalCode = await sdk.getPostalCode('postal-code-id');
```

#### Search API

```typescript
// Search across all entities
const results = await sdk.search('Lagos');
console.log('States found:', results.states.length);
console.log('LGAs found:', results.lgas.length);
console.log('Wards found:', results.wards.length);
console.log('Postal codes found:', results.postal_codes.length);
```

## Usage Examples

### React Native Component

```typescript
import React, { useState, useEffect } from 'react';
import { View, Text, FlatList, TouchableOpacity } from 'react-native';
import nigeriaGeoSDK, { State } from 'nigeria-geo-sdk';

const StatesList: React.FC = () => {
  const [states, setStates] = useState<State[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStates();
  }, []);

  const loadStates = async () => {
    try {
      const statesData = await nigeriaGeoSDK.getStates();
      setStates(statesData);
    } catch (error) {
      console.error('Error loading states:', error);
    } finally {
      setLoading(false);
    }
  };

  const renderState = ({ item }: { item: State }) => (
    <TouchableOpacity 
      style={{ padding: 16, borderBottomWidth: 1, borderBottomColor: '#eee' }}
      onPress={() => console.log('Selected state:', item.name)}
    >
      <Text style={{ fontSize: 16, fontWeight: 'bold' }}>{item.name}</Text>
      <Text style={{ fontSize: 14, color: '#666' }}>{item.code}</Text>
    </TouchableOpacity>
  );

  if (loading) {
    return (
      <View style={{ flex: 1, justifyContent: 'center', alignItems: 'center' }}>
        <Text>Loading states...</Text>
      </View>
    );
  }

  return (
    <FlatList
      data={states}
      renderItem={renderState}
      keyExtractor={(item) => item.id}
    />
  );
};

export default StatesList;
```

### Hierarchical Selection

```typescript
import React, { useState, useEffect } from 'react';
import { View, Text, Picker } from 'react-native';
import nigeriaGeoSDK, { State, Lga, Ward } from 'nigeria-geo-sdk';

const LocationPicker: React.FC = () => {
  const [states, setStates] = useState<State[]>([]);
  const [lgas, setLgas] = useState<Lga[]>([]);
  const [wards, setWards] = useState<Ward[]>([]);
  
  const [selectedState, setSelectedState] = useState<string>('');
  const [selectedLga, setSelectedLga] = useState<string>('');
  const [selectedWard, setSelectedWard] = useState<string>('');

  useEffect(() => {
    loadStates();
  }, []);

  const loadStates = async () => {
    const statesData = await nigeriaGeoSDK.getStates();
    setStates(statesData);
  };

  const loadLgas = async (stateId: string) => {
    const lgasData = await nigeriaGeoSDK.getLgasByState(stateId);
    setLgas(lgasData);
    setWards([]); // Clear wards when state changes
    setSelectedLga('');
    setSelectedWard('');
  };

  const loadWards = async (lgaId: string) => {
    const wardsData = await nigeriaGeoSDK.getWardsByLga(lgaId);
    setWards(wardsData);
    setSelectedWard('');
  };

  return (
    <View style={{ padding: 20 }}>
      <Text style={{ fontSize: 18, marginBottom: 10 }}>Select Location</Text>
      
      {/* State Picker */}
      <Picker
        selectedValue={selectedState}
        onValueChange={(value) => {
          setSelectedState(value);
          if (value) loadLgas(value);
        }}
      >
        <Picker.Item label="Select State" value="" />
        {states.map((state) => (
          <Picker.Item 
            key={state.id} 
            label={state.name} 
            value={state.id} 
          />
        ))}
      </Picker>

      {/* LGA Picker */}
      {lgas.length > 0 && (
        <Picker
          selectedValue={selectedLga}
          onValueChange={(value) => {
            setSelectedLga(value);
            if (value) loadWards(value);
          }}
        >
          <Picker.Item label="Select LGA" value="" />
          {lgas.map((lga) => (
            <Picker.Item 
              key={lga.id} 
              label={lga.name} 
              value={lga.id} 
            />
          ))}
        </Picker>
      )}

      {/* Ward Picker */}
      {wards.length > 0 && (
        <Picker
          selectedValue={selectedWard}
          onValueChange={setSelectedWard}
        >
          <Picker.Item label="Select Ward" value="" />
          {wards.map((ward) => (
            <Picker.Item 
              key={ward.id} 
              label={ward.name} 
              value={ward.id} 
            />
          ))}
        </Picker>
      )}
    </View>
  );
};

export default LocationPicker;
```

### Search Functionality

```typescript
import React, { useState } from 'react';
import { View, TextInput, FlatList, Text } from 'react-native';
import nigeriaGeoSDK, { SearchResult } from 'nigeria-geo-sdk';

const SearchComponent: React.FC = () => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSearch = async (searchQuery: string) => {
    if (searchQuery.length < 2) {
      setResults(null);
      return;
    }

    setLoading(true);
    try {
      const searchResults = await nigeriaGeoSDK.search(searchQuery);
      setResults(searchResults);
    } catch (error) {
      console.error('Search error:', error);
    } finally {
      setLoading(false);
    }
  };

  const renderResults = () => {
    if (!results) return null;

    const allResults = [
      ...results.states.map(item => ({ ...item, type: 'State' })),
      ...results.lgas.map(item => ({ ...item, type: 'LGA' })),
      ...results.wards.map(item => ({ ...item, type: 'Ward' })),
    ];

    return (
      <FlatList
        data={allResults}
        keyExtractor={(item) => `${item.type}-${item.id}`}
        renderItem={({ item }) => (
          <View style={{ padding: 12, borderBottomWidth: 1, borderBottomColor: '#eee' }}>
            <Text style={{ fontWeight: 'bold' }}>{item.name}</Text>
            <Text style={{ color: '#666' }}>{item.type} • {item.code}</Text>
          </View>
        )}
      />
    );
  };

  return (
    <View style={{ flex: 1, padding: 20 }}>
      <TextInput
        style={{
          height: 40,
          borderWidth: 1,
          borderColor: '#ddd',
          paddingHorizontal: 10,
          marginBottom: 20
        }}
        placeholder="Search states, LGAs, wards..."
        value={query}
        onChangeText={(text) => {
          setQuery(text);
          handleSearch(text);
        }}
      />
      
      {loading && <Text>Searching...</Text>}
      {renderResults()}
    </View>
  );
};

export default SearchComponent;
```

## Error Handling

```typescript
import nigeriaGeoSDK from 'nigeria-geo-sdk';

try {
  const states = await nigeriaGeoSDK.getStates();
  console.log('Success:', states);
} catch (error) {
  if (error.message.includes('timeout')) {
    console.error('Request timed out');
  } else if (error.message.includes('HTTP 404')) {
    console.error('Resource not found');
  } else if (error.message.includes('HTTP 500')) {
    console.error('Server error');
  } else {
    console.error('Network error:', error);
  }
}
```

## Configuration for Development

```typescript
import { NigeriaGeoSDK } from 'nigeria-geo-sdk';

// Development configuration
const devSDK = NigeriaGeoSDK.getInstance({
  baseUrl: 'http://localhost:3000',  // Local development server
  timeout: 10000,                   // Longer timeout for development
  enableCaching: false,             // Disable caching for fresh data
  enableLogging: true,              // Enable logging for debugging
});
```

## Performance Tips

1. **Use Pagination**: The SDK supports limit parameters to control data size
2. **Enable Caching**: Keep caching enabled for better performance
3. **Debounce Search**: Implement search debouncing to avoid excessive API calls
4. **Error Boundaries**: Use React Error Boundaries for graceful error handling

```typescript
// Example: Debounced search
import { debounce } from 'lodash';

const debouncedSearch = debounce(async (query: string) => {
  const results = await nigeriaGeoSDK.search(query);
  setSearchResults(results);
}, 300);

// Use in component
const handleSearchInput = (text: string) => {
  setQuery(text);
  debouncedSearch(text);
};
```

## Compatibility

- **React Native**: 0.60.0+
- **React**: 16.8.0+
- **Expo**: SDK 40+
- **TypeScript**: 4.0+
- **Metro**: Default React Native bundler

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
- Enhanced pagination support with configurable limits
- Updated production server configuration
- Improved TypeScript definitions

### v1.0.0
- Initial release
- Complete Nigerian geographic data coverage
- Zero-configuration setup
- Full TypeScript support
- Search functionality across all entities

---

Made with ❤️ for Nigerian developers