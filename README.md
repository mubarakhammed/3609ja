# 🇳🇬 Nigeria Geographic API

> A ## Comprehensive Data Coverage

| **Geographic Level** | **Count** | **Coverage** |
|---------------------|-----------||--------------|
| States            | 37        | 100% (All Nigerian states + FCT) |
| Local Government Areas | 774   | 100% (Authentic LGA data) |
| Wards             | 7,858     | Comprehensive ward coverage |
| Postal Codes     | 7,858     | One per ward, Nigerian postal system |formance, production-ready Rust API providing comprehensive Nigerian geographic data, postal codes, and address validation services with mobile SDKs.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![API Version](https://img.shields.io/badge/API-v1-blue)]()
[![Rust](https://img.shields.io/badge/rust-1.70+-orange)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()

## What Makes This Special

- **Complete Nigerian Data** - All 37 states, 774 LGAs, 7,858 wards & postal codes
- **High Performance** - Built with Rust + Redis caching for blazing speed
- **Mobile SDKs** - Flutter, React Native, iOS & Android SDKs included
- **Production Ready** - API usage tracking, monitoring, and enterprise features
- **Clean Architecture** - Domain-driven design with proper separation of concerns
- **Real-Time Analytics** - Comprehensive API usage tracking and analytics
- **RESTful API** - Well-documented endpoints with OpenAPI/Swagger support

## Live API Demo

**Production API:** `http://20.63.52.179:3000`

Try it now:
```bash
# Get all Nigerian states
curl http://20.63.52.179:3000/api/v1/states

# Search for locations
curl "http://20.63.52.179:3000/api/v1/search?q=Lagos"

# Get LGAs in a state
curl http://20.63.52.179:3000/api/v1/states/{state-id}/lgas
```

## 📊 Comprehensive Data Coverage

| **Geographic Level** | **Count** | **Coverage** |
|---------------------|-----------|--------------|
| 🏛️ States            | 37        | 100% (All Nigerian states + FCT) |
| 🏘️ Local Government Areas | 774   | 100% (Authentic LGA data) |
| 🏡 Wards             | 7,858     | Comprehensive ward coverage |
| 📮 Postal Codes     | 7,858     | One per ward, Nigerian postal system |

## Quick Start (3 minutes)

### Prerequisites
- Rust 1.70+ 
- PostgreSQL 14+
- Redis 6+ (optional, for caching)

### Setup & Run

```bash
# 1. Clone the repository
git clone https://github.com/mubarakhammed/3609ja.git
cd 3609ja

# 2. Set up environment
cp .env.example .env
# Edit .env with your database credentials

# 3. Install dependencies and run
cargo build --release
cargo run --release

# 4. Test it works
curl http://localhost:3000/api/v1/health
```

### Production Deployment

```bash
# Use the deployment script
chmod +x deploy.sh
./deploy.sh

# Or build manually
cargo build --release
./target/release/nigeria-geo-api
```

## Mobile SDKs

### Flutter SDK
```dart
dependencies:
  nigeria_geo_sdk: ^1.0.0

// Usage
final sdk = NigeriaGeoSDK();
final states = await sdk.getStates();
final lgas = await sdk.getLGAsByState(stateId);
```

### React Native SDK
```bash
npm install nigeria-geo-sdk

// Usage
import { NigeriaGeoSDK } from 'nigeria-geo-sdk';
const sdk = new NigeriaGeoSDK();
const states = await sdk.getStates();
```

### iOS SDK
```swift
dependencies: [
    .package(url: "https://github.com/mubarakhammed/3609ja", from: "1.0.0")
]

// Usage
let sdk = NigeriaGeoSDK()
let states = try await sdk.getStates()
```

## API Endpoints

### Geographic Data
```
GET  /api/v1/states                    # Get all states
GET  /api/v1/states/{id}              # Get state by ID
GET  /api/v1/states/{id}/lgas         # Get LGAs in state
GET  /api/v1/lgas/{id}                # Get LGA by ID
GET  /api/v1/lgas/{id}/wards          # Get wards in LGA
GET  /api/v1/wards/{id}               # Get ward by ID
GET  /api/v1/wards/{id}/postal-codes  # Get postal codes in ward
```

### Search & Discovery
```
GET  /api/v1/search                   # Search all geographic data
GET  /api/v1/search/states           # Search states
GET  /api/v1/search/lgas             # Search LGAs
GET  /api/v1/search/wards            # Search wards
GET  /api/v1/postal-codes/nearby     # Find nearby postal codes
```

### Address Validation
```
POST /api/v1/validate                # Validate Nigerian address
GET  /api/v1/address/find            # Find address by components
POST /api/v1/address/similar         # Find similar addresses
```


## Architecture

```
src/
├── api/                    # API handlers & analytics
├── application/           # Use cases & DTOs
│   ├── use_cases/           # Business logic
│   └── dtos/               # Data transfer objects
├── domain/               # Domain models & repositories
│   ├── entities/           # Core business entities
│   ├── repositories/       # Repository interfaces
│   └── value_objects/      # Domain value objects
├── infrastructure/       # External concerns
│   ├── database/           # Database implementations
│   ├── cache/             # Redis caching
│   └── repositories/      # Repository implementations
└── presentation/        # HTTP layer
    ├── handlers/          # Request handlers
    ├── middleware/        # Request middleware
    └── routes/           # Route definitions
```

## Database Schema

```sql
-- Core geographic hierarchy
states (37 records)
├── lgas (774 records)
    ├── wards (7,858 records)
        └── postal_codes (7,858 records)

-- Additional features
├── addresses           # Address validation
├── api_usage          # Usage tracking
└── api_usage_hourly   # Analytics aggregation
```

## Development Setup

### Local Development
```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Clone and setup
git clone https://github.com/mubarakhammed/3609ja.git
cd 3609ja
cp .env.example .env

# 3. Setup PostgreSQL
createdb nigeria_geo
sqlx migrate run

# 4. Populate data
python3 scripts/populate_production_db.py
python3 scripts/generate_postal_codes.py

# 5. Run development server
cargo run
```

### Testing
```bash
# Run tests
cargo test

# Run with coverage
cargo test --coverage

# Load testing
./scripts/test_api.sh
```

## Performance Features

- **Redis Caching** - Sub-millisecond response times
- **Connection Pooling** - Efficient database connections  
- **Async Processing** - Non-blocking I/O operations
- **Usage Analytics** - Real-time API monitoring
- **Rate Limiting** - Protection against abuse
- **Health Checks** - Automated monitoring

## Production Deployment


### Environment Variables
```bash
# Database
DATABASE_URL=postgres://user:pass@host:5432/nigeria_geo

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Caching (optional)
REDIS_URL=redis://localhost:6379

# Security
JWT_SECRET=your-secure-secret
CORS_ORIGIN=https://yourdomain.com
```

## API Usage Analytics

Monitor your API with comprehensive analytics:

- **Request Volume** - Track API calls over time
- **Response Times** - Monitor performance metrics  
- **Top Endpoints** - Identify most popular endpoints
- **Error Rates** - Track and debug issues
- **Geographic Usage** - See where requests originate

## Contributing

We welcome contributions! Here's how:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines
- Follow Rust conventions and clippy suggestions
- Add tests for new features
- Update documentation for API changes
- Ensure backward compatibility

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] **GraphQL API** - Alternative query interface
- [ ] **WebSocket Support** - Real-time updates
- [ ] **Machine Learning** - Address prediction & correction
- [ ] **Geocoding** - Latitude/longitude coordinates
- [ ] **Batch Operations** - Bulk data processing
- [ ] **Admin Dashboard** - Web-based management interface

## Acknowledgments

- **Nigerian Postal Service** - For postal code standards
- **INEC** - For official geographic boundaries  
- **Open Source Community** - For amazing Rust crates
- **Contributors** - For making this project better

## Support & Contact

- **Issues:** [GitHub Issues](https://github.com/mubarakhammed/3609ja/issues)
- **Discussions:** [GitHub Discussions](https://github.com/mubarakhammed/3609ja/discussions)
- **Email:** [mubarak@example.com](mailto:mubarak@example.com)

---

<div align="center">

**Star this repo if it helped you!**

[Star](https://github.com/mubarakhammed/3609ja) • [Fork](https://github.com/mubarakhammed/3609ja/fork) • [Docs](https://github.com/mubarakhammed/3609ja#readme) • [Issues](https://github.com/mubarakhammed/3609ja/issues)

</div>



