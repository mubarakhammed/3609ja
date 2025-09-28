# Changelog

All notable changes to the Nigeria Geo SDK will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-09-28

### Added
- High-performance React Native SDK for Nigerian geographic data
- Complete TypeScript support with full type definitions
- Zero-configuration setup with sensible production defaults
- Comprehensive API coverage:
  - All 37 Nigerian states
  - 774 Local Government Areas (LGAs) 
  - 8,840+ Electoral wards
  - Postal codes database
- Real-time search functionality across all geographic entities
- Built-in intelligent caching system
- Cross-platform support (React Native, Expo, bare RN projects)

### Performance
- Optimized API endpoints with < 500ms response times
- Direct database access architecture (removed caching bottlenecks)
- Reduced timeout settings (5s vs 15s default)
- Minimal bundle size impact (5.6kB package)

### Developer Experience
- Comprehensive documentation with examples
- Full example React Native app with UI components
- Hierarchical dropdown components (State → LGA → Ward)
- Address form builders
- Search interface components
- TypeScript IntelliSense support

### Technical
- Production server deployment at optimized API endpoint
- Robust error handling and timeout management
- Smart caching with configurable expiration
- Support for both development and production environments
- Comprehensive test suite

### Documentation
- Complete README with usage examples
- API reference documentation
- TypeScript interface definitions
- Example React Native application
- Installation and setup guides

## [1.0.0] - Initial Release
- Basic SDK functionality
- Core API endpoints
- TypeScript support