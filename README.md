# 🇳🇬 Nigeria Geographic API

> A high-performance, open-source Rust API providing comprehensive Nigerian geographic data, postal codes, and address validation services.

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![PostgreSQL](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)
[![Redis](https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white)](https://redis.io/)

## 🌟 Welcome Contributors!

We're building Nigeria's most comprehensive geographic API, and **we need your help**! Whether you're a Rust expert, a Nigerian geography enthusiast, or just someone who wants to contribute to open source, there's a place for you here.

**What makes this special:**
- 🏗️ **Clean Architecture** - Well-structured, maintainable codebase
- ⚡ **High Performance** - Built with Rust for speed and safety
- 🗺️ **Comprehensive Data** - Complete Nigerian geographic hierarchy
- 🧪 **Production Ready** - With caching, monitoring, and Docker support

## 🚀 Quick Start (60 seconds)

```bash
# 1. Clone the repo
git clone https://github.com/mubarakhammed/3609ja.git
cd 3609ja

# 2. Start with Docker (easiest way)
docker-compose up -d

# 3. Test it works
curl http://localhost:3000/api/v1/health
```

That's it! The API is now running with complete Nigerian geographic data.

## �️ What This API Provides

- **37 States** - All Nigerian states + FCT
- **143+ LGAs** - Complete Local Government Areas 
- **40+ Wards** - Electoral wards with boundaries
- **48+ Postal Codes** - With real GPS coordinates
- **Address Validation** - Smart validation with suggestions
- **Geographic Search** - Fast autocomplete across all entities
- **Proximity Queries** - Find nearby locations by coordinates

## 🏗️ Codebase Navigation

Our codebase follows **Clean Architecture** principles. Here's how to navigate:

```
src/
├── 🏛️  domain/           # Business logic (entities, repositories)
│   ├── entities/         # Core business objects (State, LGA, Ward, etc.)
│   ├── repositories/     # Data access interfaces 
│   └── value_objects/    # Domain-specific types
│
├── 🎯 application/       # Use cases and business workflows
│   ├── use_cases/        # Business operations
│   └── dtos/            # Data transfer objects
│
├── 🔌 infrastructure/    # External concerns (database, cache, etc.)
│   ├── database/         # Database implementations
│   ├── cache.rs         # Redis caching layer
│   └── repositories/     # Repository implementations
│
├── 🌐 presentation/      # HTTP layer (handlers, routes, middleware)
│   ├── handlers.rs      # API endpoint handlers
│   ├── routes.rs        # Route definitions
│   └── middleware.rs    # Request/response middleware
│
└── 📋 config.rs         # Configuration management
```

**Key files to understand:**
- `src/main.rs` - Application entry point
- `src/presentation/handlers.rs` - API endpoints
- `src/infrastructure/cache.rs` - Redis caching logic
- `migrations/` - Database schema
- `scripts/` - Data seeding scripts

## 🛠️ Development Setup

### Option 1: Docker (Recommended)
```bash
# Clone and start everything
git clone https://github.com/mubarakhammed/3609ja.git
cd 3609ja
cp env.example .env
docker-compose up -d

# Seed data
docker-compose exec postgres psql -U nigeria_user -d nigeria_geo -f /scripts/seed_complete_master.sql
```

### Option 2: Local Development
```bash
# Prerequisites: Rust 1.75+, PostgreSQL 15+, Redis (optional)
git clone https://github.com/mubarakhammed/3609ja.git
cd 3609ja

# Setup database
createdb nigeria_geo
psql -c "CREATE USER nigeria_user WITH PASSWORD 'nigeria_password';"
psql -c "GRANT ALL PRIVILEGES ON DATABASE nigeria_geo TO nigeria_user;"

# Setup environment
cp env.example .env
# Edit .env with your database credentials

# Install SQLx CLI and run migrations
cargo install sqlx-cli
sqlx migrate run

# Seed data
psql -U nigeria_user -d nigeria_geo -f scripts/seed_complete_master.sql

# Run the application
cargo run
```

### Verify Installation
```bash
# Test endpoints
curl http://localhost:3000/api/v1/health
curl http://localhost:3000/api/v1/states
curl "http://localhost:3000/api/v1/search?query=lagos"
```

## 🤝 How to Contribute

We welcome contributions of all sizes! Here's how to get started:

### 1. 🐛 Report Issues
- Found a bug? [Create an issue](../../issues/new)
- Missing data for your area? [Let us know](../../issues/new)
- Performance problems? [Tell us about it](../../issues/new)

### 2. 📝 Documentation
- Improve API documentation
- Add code comments
- Write tutorials or guides
- Update this README

### 3. 🔧 Code Contributions

**Easy Tasks (Great for first-time contributors):**
- Add more test cases
- Fix compiler warnings
- Improve error messages
- Add validation rules

**Medium Tasks:**
- Implement rate limiting
- Add more endpoints
- Improve caching strategies
- Add monitoring metrics

**Advanced Tasks:**
- Performance optimizations
- Add authentication/authorization
- Implement WebSocket support
- Add geographic visualization

### 4. 📊 Data Contributions
- Add missing LGAs or Wards
- Verify postal codes
- Add GPS coordinates
- Improve address validation rules

## 📋 Contribution Guidelines

### Before You Start
1. **Check existing issues** to avoid duplicate work
2. **Create an issue** to discuss major changes
3. **Fork the repository** and create a feature branch

### Code Standards
- **Rust formatting**: Use `cargo fmt`
- **Linting**: Run `cargo clippy -- -D warnings`
- **Testing**: Add tests for new features (`cargo test`)
- **Documentation**: Update docs for public APIs
- **Commits**: Use clear, descriptive commit messages

### Pull Request Process
1. **Create a feature branch**: `git checkout -b feature/amazing-feature`
2. **Make your changes** following our code standards
3. **Add tests** for new functionality
4. **Update documentation** if needed
5. **Run the test suite**: `cargo test`
6. **Submit a PR** with a clear description

### Review Process
- All PRs require at least one review
- Automated checks must pass (formatting, linting, tests)
- Be responsive to feedback
- Maintain a respectful, collaborative tone

## � API Documentation

### Core Endpoints
```bash
# States
GET /api/v1/states                     # List all states
GET /api/v1/states/{id}               # Get state by ID
GET /api/v1/states/{id}/lgas          # Get LGAs for a state

# LGAs  
GET /api/v1/lgas/{id}                 # Get LGA by ID
GET /api/v1/lgas/{id}/wards           # Get wards for an LGA

# Postal Codes
GET /api/v1/postal-codes/{code}       # Get by postal code
GET /api/v1/postal-codes/nearby       # Find nearby (lat, lng, radius)

# Search & Validation
GET /api/v1/search?query=lagos        # Search all entities
POST /api/v1/validate                 # Validate address
```

### Response Format
```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 37,
    "total_pages": 2,
    "has_next": true,
    "has_prev": false
  }
}
```

## 🚢 Production Deployment

### Docker Production
```bash
# Build and run
docker build -t nigeria-geo-api .
docker run -p 3000:3000 --env-file .env nigeria-geo-api
```

### Environment Variables
```env
# Required
DATABASE_URL=postgres://user:pass@localhost:5432/nigeria_geo
REDIS_URL=redis://localhost:6379

# Optional
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
RUST_LOG=info
```

## 🔧 Architecture Decisions

### Why Rust?
- **Performance**: Near C++ speed with memory safety
- **Concurrency**: Built-in async/await support
- **Type Safety**: Prevents entire classes of bugs
- **Ecosystem**: Great HTTP and database libraries

### Why Clean Architecture?
- **Testability**: Easy to unit test business logic
- **Maintainability**: Clear separation of concerns
- **Flexibility**: Easy to swap out infrastructure
- **Scalability**: Well-structured for growth

### Why PostgreSQL + Redis?
- **PostgreSQL**: ACID compliance, great for geographic data
- **Redis**: High-performance caching, reduces database load
- **Combined**: Best of both worlds for read-heavy workloads

## 🎯 Roadmap

### Current Focus
- [ ] Complete address validation system
- [ ] Comprehensive test coverage (>90%)
- [ ] API rate limiting
- [ ] OpenAPI documentation

### Future Vision
- [ ] Machine learning address correction
- [ ] Real-time geographic updates
- [ ] Mobile SDKs (iOS, Android)
- [ ] Geographic visualization dashboard
- [ ] Webhook notification system

## 🆘 Getting Help

- **Documentation**: Check our [API docs](API_DOCUMENTATION.md)
- **Issues**: [Search existing issues](../../issues) first
- **Discussions**: Use [GitHub Discussions](../../discussions) for questions
- **Discord**: Join our community server (coming soon)

## 🏆 Recognition

Contributors will be recognized in:
- This README file
- Release notes
- Project website (coming soon)
- Annual contributor spotlight

**Top Contributors:**
- @mubarakhammed (Mubarak) - Project maintainer
- *Your name could be here!*

## � License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

---

**Ready to contribute?** Start with a [good first issue](../../labels/good%20first%20issue) and join our community of developers building Nigeria's digital infrastructure! 🚀