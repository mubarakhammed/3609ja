# Nigeria Geo React Native SDK

A React Native SDK for accessing Nigerian geographic data including states, LGAs, wards, and postal codes.

## Installation

```bash
npm install @nigeriageo/react-native-sdk
# or
yarn add @nigeriageo/react-native-sdk
```

## Configuration

### Using Config File

1. Copy the example config file:
```bash
cp assets/config.yaml.example assets/config.yaml
```

2. Update the configuration values in `assets/config.yaml`:
```yaml
development:
  baseUrl: "http://localhost:3000"
  timeout: 30000
  enableCaching: true
  enableLogging: true

production:
  baseUrl: "https://your-production-api.com"
  timeout: 15000
  enableCaching: true
  enableLogging: false
```

### Using Environment Variables

Set `NODE_ENV` to automatically select the appropriate configuration:
```bash
NODE_ENV=production # Uses production config
NODE_ENV=staging    # Uses staging config  
NODE_ENV=development # Uses development config (default)
```

### Programmatic Configuration

```typescript
import { NigeriaGeoSDK } from '@nigeriageo/react-native-sdk';

// Zero-config usage (uses development defaults)
const sdk = NigeriaGeoSDK.getInstance();

// With custom config
const sdk = NigeriaGeoSDK.getInstance({
  baseUrl: 'https://api.example.com',
  timeout: 20000,
  enableLogging: false
});

// With environment specification
const sdk = NigeriaGeoSDK.getInstance({}, 'production');

// Reconfigure existing instance
await NigeriaGeoSDK.reconfigure({
  baseUrl: 'https://new-api.example.com'
}, 'staging');
```

## Usage

```typescript
import nigeriaGeoSDK from '@nigeriageo/react-native-sdk';

// Get all states
const states = await nigeriaGeoSDK.getStates();

// Get LGAs for a state
const lgas = await nigeriaGeoSDK.getLgasByState(stateId);

// Get wards for an LGA
const wards = await nigeriaGeoSDK.getWardsByLga(lgaId);

// Search functionality
const results = await nigeriaGeoSDK.search('Lagos');
```

## Configuration Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `baseUrl` | string | `http://localhost:3000` | API base URL |
| `timeout` | number | `30000` | Request timeout in milliseconds |
| `enableCaching` | boolean | `true` | Enable response caching |
| `enableLogging` | boolean | `true` | Enable error logging |

## Environment-Specific Defaults

- **Development**: Full logging, localhost API, long timeout
- **Staging**: Logging enabled, staging API, medium timeout  
- **Production**: Logging disabled, production API, short timeout

## Security Note

Never commit your actual `config.yaml` file with production API keys or sensitive URLs. Always use the example file as a template and configure your actual values locally or through environment variables.