# Nigeria Geo SDK for React Native

[![npm version](https://img.shields.io/npm/v/nigeria-geo-sdk)](https://www.npmjs.com/package/nigeria-geo-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/%3C%2F%3E-TypeScript-%230074c1.svg)](http://www.typescriptlang.org/)

A high-performance React Native SDK for accessing comprehensive Nigerian geographic data including states, LGAs, wards, and postal codes. Optimized for fast response times with direct database access.

## ✨ Features

- 🇳🇬 **Complete Nigerian Geographic Data**: All 37 states, 774 LGAs, 8,840+ wards, and postal codes
- ⚡ **High Performance**: Optimized API with < 500ms response times
- 📱 **Cross Platform**: Works with React Native, Expo, and bare React Native projects
- 🎯 **Zero Configuration**: Works out of the box with sensible defaults
- 🔍 **Powerful Search**: Full-text search across all geographic entities
- 📍 **Location Services**: Geographic data with postal codes
- 🎨 **TypeScript Support**: Full TypeScript definitions included
- 💾 **Smart Caching**: Built-in caching for better performance
- 🔒 **Type Safe**: Complete TypeScript support with strong typing

## 📦 Installation

```bash
npm install nigeria-geo-sdk
```

### With Yarn
```bash
yarn add nigeria-geo-sdk
```

### With Expo
```bash
expo install nigeria-geo-sdk
```

## 🚀 Quick Start

### Basic Usage (Zero Configuration)

```typescript
import nigeriaGeoSDK from 'nigeria-geo-sdk';

// Get all Nigerian states
const states = await nigeriaGeoSDK.getStates();
console.log('States:', states);

// Get LGAs in a state
const lgas = await nigeriaGeoSDK.getLgasByState(states[0].id);
console.log('LGAs:', lgas);

// Get wards in an LGA
const wards = await nigeriaGeoSDK.getWardsByLga(lgas[0].id);
console.log('Wards:', wards);

// Search for locations
const results = await nigeriaGeoSDK.search('Lagos');
console.log('Search results:', results);
```

### Complete React Native Example

```typescript
import React, { useState, useEffect } from 'react';
import { View, Text, ScrollView } from 'react-native';
import nigeriaGeoSDK, { State, Lga, Ward } from 'nigeria-geo-sdk';

export default function App() {
  const [states, setStates] = useState<State[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadStates = async () => {
      try {
        const data = await nigeriaGeoSDK.getStates();
        setStates(data);
      } catch (error) {
        console.error('Error loading states:', error);
      } finally {
        setLoading(false);
      }
    };

    loadStates();
  }, []);

  if (loading) {
    return <Text>Loading...</Text>;
  }

  return (
    <ScrollView>
      <Text style={{ fontSize: 18, fontWeight: 'bold' }}>
        Nigerian States ({states.length})
      </Text>
      {states.map((state) => (
        <Text key={state.id}>{state.name} ({state.code})</Text>
      ))}
    </ScrollView>
  );
}
```

## 📚 API Reference

### Methods

#### `getStates(): Promise<State[]>`
Retrieves all Nigerian states.

```typescript
const states = await nigeriaGeoSDK.getStates();
```

#### `getLgasByState(stateId: string): Promise<Lga[]>`
Retrieves all LGAs for a specific state.

```typescript
const lgas = await nigeriaGeoSDK.getLgasByState('state-id');
```

#### `getWardsByLga(lgaId: string): Promise<Ward[]>`
Retrieves all wards for a specific LGA.

```typescript
const wards = await nigeriaGeoSDK.getWardsByLga('lga-id');
```

#### `search(query: string): Promise<SearchResult>`
Searches across all geographic data.

```typescript
const results = await nigeriaGeoSDK.search('Lagos');
// Returns: { states: [], lgas: [], wards: [], postal_codes: [] }
```

#### `getPostalCodes(params?: PostalCodeParams): Promise<PostalCode[]>`
Retrieves postal codes with optional filtering.

```typescript
const postalCodes = await nigeriaGeoSDK.getPostalCodes({
  state: 'Lagos',
  limit: 10
});
```

### Types

```typescript
interface State {
  id: string;
  name: string;
  code: string;
  created_at: string;
  updated_at: string;
}

interface Lga {
  id: string;
  state_id: string;
  name: string;
  code: string;
  created_at: string;
  updated_at: string;
}

interface Ward {
  id: string;
  lga_id: string;
  name: string;
  code: string;
  created_at: string;
  updated_at: string;
}

interface PostalCode {
  id: string;
  code: string;
  area: string;
  state: string;
  lga?: string;
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

## ⚙️ Configuration

The SDK works with zero configuration but can be customized:

```typescript
import { ConfigLoader } from 'nigeria-geo-sdk';

// Load custom configuration
const config = await ConfigLoader.loadConfig('production');

// Custom configuration
const customConfig = {
  baseUrl: 'https://your-api-endpoint.com',
  timeout: 10000,
  enableCaching: true,
  enableLogging: false
};
```

## 🎨 UI Components Example

Create reusable dropdown components:

```typescript
import React, { useState, useEffect } from 'react';
import { Picker } from '@react-native-picker/picker';
import nigeriaGeoSDK, { State, Lga, Ward } from 'nigeria-geo-sdk';

function StateDropdown({ onStateChange }) {
  const [states, setStates] = useState<State[]>([]);

  useEffect(() => {
    nigeriaGeoSDK.getStates().then(setStates);
  }, []);

  return (
    <Picker onValueChange={onStateChange}>
      <Picker.Item label="Select a state..." value="" />
      {states.map((state) => (
        <Picker.Item key={state.id} label={state.name} value={state.id} />
      ))}
    </Picker>
  );
}
```

## 🚀 Performance

- **Response Times**: < 500ms for all API calls
- **Data Coverage**: 100% of Nigerian geographic entities
- **Caching**: Built-in intelligent caching system
- **Bundle Size**: Minimal impact on your app size
- **Offline Support**: Cached data available offline

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Support

⭐ If this SDK helped you, please give it a star on [GitHub](https://github.com/mubarakhammed/3609ja)!

[![GitHub Sponsors](https://img.shields.io/github/sponsors/mubarakhammed?style=social)](https://github.com/sponsors/mubarakhammed)

## 📊 Data Sources

This SDK provides access to official Nigerian geographic data including:
- Federal Capital Territory and 36 States
- 774 Local Government Areas (LGAs)
- 8,840+ Electoral Wards
- Comprehensive postal codes database

---

Made with ❤️ in Nigeria 🇳🇬