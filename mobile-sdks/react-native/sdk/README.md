# Nigeria Geo SDK - React Native

Zero-configuration React Native SDK for Nigerian geographical data. Get instant access to states, LGAs, wards, and postal codes with ready-to-use UI components.

## Features

- 🚀 **Zero Configuration** - Works out of the box with production API
- 📱 **React Native Ready** - Built specifically for React Native and Expo
- 🎨 **UI Components** - Pre-built dropdowns with Material Design
- 🔍 **Smart Search** - Search across all geographical data
- ✅ **Address Validation** - Validate address components
- 📦 **TypeScript Support** - Full type safety and IntelliSense
- 🌐 **Hierarchical Data** - State → LGA → Ward relationships
- 📮 **Postal Codes** - Complete postal code database

## Installation

```bash
npm install nigeria-geo-sdk @react-native-picker/picker react-native-paper
# or
yarn add nigeria-geo-sdk @react-native-picker/picker react-native-paper
```

For Expo projects, also install:
```bash
expo install react-native-vector-icons
```

## Quick Start

### Zero-Config Usage

```typescript
import NigeriaGeoSDK, { StateDropdown } from 'nigeria-geo-sdk';

// Get all states instantly
const states = await NigeriaGeoSDK.getStates();

// Use pre-built UI component
function MyComponent() {
  const [selectedState, setSelectedState] = useState(null);
  
  return (
    <StateDropdown
      onStateChange={setSelectedState}
      selectedState={selectedState}
    />
  );
}
```

## API Reference

### Core SDK Methods

```typescript
import { NigeriaGeoSDK } from 'nigeria-geo-sdk';

const sdk = NigeriaGeoSDK.getInstance();

// States
const states = await sdk.getStates();
const state = await sdk.getState(25); // Lagos

// Local Government Areas
const lgas = await sdk.getLgas(25); // LGAs in Lagos
const lga = await sdk.getLga(317); // Specific LGA

// Wards
const wards = await sdk.getWards(317); // Wards in LGA
const ward = await sdk.getWard(8154); // Specific ward

// Postal Codes
const postalCodes = await sdk.getPostalCodes(25); // Postal codes in Lagos
const postalCode = await sdk.getPostalCode(1245);

// Search
const results = await sdk.search('ikeja');

// Validation
const isValid = await sdk.validateAddress('Lagos', 'Ikeja', 'Allen', '100001');
```

### UI Components

#### StateDropdown
```typescript
import { StateDropdown } from 'nigeria-geo-sdk';

<StateDropdown
  onStateChange={(state) => console.log(state)}
  selectedState={selectedState}
  placeholder="Choose State"
  style={customStyle}
/>
```

#### LgaDropdown
```typescript
import { LgaDropdown } from 'nigeria-geo-sdk';

<LgaDropdown
  stateId={selectedState?.id}
  onLgaChange={(lga) => console.log(lga)}
  selectedLga={selectedLga}
  placeholder="Choose LGA"
/>
```

#### WardDropdown
```typescript
import { WardDropdown } from 'nigeria-geo-sdk';

<WardDropdown
  lgaId={selectedLga?.id}
  onWardChange={(ward) => console.log(ward)}
  selectedWard={selectedWard}
  placeholder="Choose Ward"
/>
```

#### PostalCodeDropdown
```typescript
import { PostalCodeDropdown } from 'nigeria-geo-sdk';

<PostalCodeDropdown
  stateId={selectedState?.id}
  onPostalCodeChange={(postal) => console.log(postal)}
  selectedPostalCode={selectedPostalCode}
  placeholder="Choose Postal Code"
/>
```

## Complete Example

```typescript
import React, { useState } from 'react';
import { View, StyleSheet } from 'react-native';
import { Button, Text, Card } from 'react-native-paper';
import { 
  StateDropdown, 
  LgaDropdown, 
  WardDropdown,
  NigeriaGeoSDK 
} from 'nigeria-geo-sdk';
import type { State, Lga, Ward } from 'nigeria-geo-sdk';

export default function AddressForm() {
  const [selectedState, setSelectedState] = useState<State | null>(null);
  const [selectedLga, setSelectedLga] = useState<Lga | null>(null);
  const [selectedWard, setSelectedWard] = useState<Ward | null>(null);
  const [isValidating, setIsValidating] = useState(false);
  const [isValid, setIsValid] = useState<boolean | null>(null);

  const validateAddress = async () => {
    if (!selectedState) return;
    
    setIsValidating(true);
    try {
      const sdk = NigeriaGeoSDK.getInstance();
      const valid = await sdk.validateAddress(
        selectedState.name,
        selectedLga?.name,
        selectedWard?.name
      );
      setIsValid(valid);
    } catch (error) {
      console.error('Validation error:', error);
    } finally {
      setIsValidating(false);
    }
  };

  return (
    <Card style={styles.card}>
      <Card.Title title="Address Selection" />
      <Card.Content>
        <StateDropdown
          onStateChange={(state) => {
            setSelectedState(state);
            setSelectedLga(null);
            setSelectedWard(null);
            setIsValid(null);
          }}
          selectedState={selectedState}
        />
        
        <LgaDropdown
          stateId={selectedState?.id}
          onLgaChange={(lga) => {
            setSelectedLga(lga);
            setSelectedWard(null);
            setIsValid(null);
          }}
          selectedLga={selectedLga}
        />
        
        <WardDropdown
          lgaId={selectedLga?.id}
          onWardChange={(ward) => {
            setSelectedWard(ward);
            setIsValid(null);
          }}
          selectedWard={selectedWard}
        />
        
        <Button 
          mode="contained"
          onPress={validateAddress}
          loading={isValidating}
          disabled={!selectedState}
          style={styles.button}
        >
          Validate Address
        </Button>
        
        {isValid !== null && (
          <Text style={[styles.result, isValid ? styles.valid : styles.invalid]}>
            {isValid ? '✅ Valid Address' : '❌ Invalid Address'}
          </Text>
        )}
      </Card.Content>
    </Card>
  );
}

const styles = StyleSheet.create({
  card: {
    margin: 16,
  },
  button: {
    marginTop: 16,
  },
  result: {
    marginTop: 12,
    textAlign: 'center',
    fontWeight: 'bold',
  },
  valid: {
    color: '#4caf50',
  },
  invalid: {
    color: '#f44336',
  },
});
```

## TypeScript Types

```typescript
interface State {
  id: number;
  name: string;
  code: string;
  capital: string;
  region: string;
  created_at: string;
  updated_at: string;
}

interface Lga {
  id: number;
  name: string;
  code: string;
  state_id: number;
  created_at: string;
  updated_at: string;
}

interface Ward {
  id: number;
  name: string;
  code: string;
  lga_id: number;
  created_at: string;
  updated_at: string;
}

interface PostalCode {
  id: number;
  code: string;
  area: string;
  district: string;
  state_id: number;
  created_at: string;
  updated_at: string;
}
```

## Configuration

### Custom API Endpoint
```typescript
import { NigeriaGeoSDK } from 'nigeria-geo-sdk';

// Use custom API endpoint
const sdk = NigeriaGeoSDK.getInstance('https://your-api-endpoint.com');
```

### Error Handling
```typescript
try {
  const states = await sdk.getStates();
} catch (error) {
  if (error instanceof Error) {
    console.error('API Error:', error.message);
  }
}
```

## Requirements

- React Native 0.70+
- Expo SDK 49+ (for Expo projects)
- TypeScript 4.0+ (recommended)

## Demo App

This package includes a comprehensive demo app showcasing all features:

```bash
git clone <repo-url>
cd mobile-sdks/react-native/nigeria-geo-sdk
npm install
npm run ios # or npm run android
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see LICENSE file for details.

## Support

- 📖 [API Documentation](https://nigeria-geo-api.onrender.com/docs)
- 🐛 [Report Issues](https://github.com/your-repo/issues)
- 💬 [Discussions](https://github.com/your-repo/discussions)

---

Made with ❤️ for Nigerian developers