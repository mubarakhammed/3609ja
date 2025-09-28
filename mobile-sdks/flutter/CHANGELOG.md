# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2024-12-28

### Added
- Performance optimizations with direct database access architecture
- Enhanced widget pickers to show comprehensive data (50+ states, 50+ LGAs, 100+ wards)
- Production server configuration (http://20.63.52.179:3000)
- Comprehensive README with detailed usage examples
- New pagination demonstration in example app
- Support for all 37 Nigerian states, 774 LGAs, and 8,840+ wards

### Changed
- **BREAKING**: Reduced default API timeout from 30s to 5s for optimal performance
- Updated cache expiration from 24h to 6h due to improved API speed
- Reduced default cache size from 50MB to 25MB
- Widget pickers now fetch more comprehensive data instead of limiting to 20 items
- Updated example app with better performance demonstrations

### Improved
- API response times now < 500ms (down from 4+ minutes)
- Better error handling and retry mechanisms
- Enhanced search functionality with more comprehensive results
- Improved caching strategy for optimal performance

### Fixed
- Removed artificial limits in widget pickers
- Fixed pagination handling in example app
- Improved memory management with optimized cache settings

## [1.0.0] - 2024-12-01

### Added
- Initial release of Nigeria Geo SDK for Flutter
- Complete Nigerian geographic data coverage
- Full Flutter integration with pre-built widgets
- Comprehensive search and validation features
- Address validation and suggestion system
- Location-based postal code finder
- Hierarchical selection widgets (State → LGA → Ward)
- Caching support for offline capabilities
- Stream-based reactive APIs
- Complete example app demonstrating all features
- Support for all Flutter platforms (iOS, Android, Web, Desktop)
- Type-safe models with null safety support
- Comprehensive error handling
- Built-in retry mechanisms for network requests
- Configurable logging and debugging support

### Features
- **States API**: Get, search, and paginate through all 37 Nigerian states
- **LGAs API**: Access all 774 Local Government Areas with state relationships
- **Wards API**: Complete ward data with LGA relationships
- **Postal Codes API**: Geographic postal codes with coordinates
- **Search API**: Full-text search across all geographic entities
- **Address Validation**: Validate and get suggestions for Nigerian addresses
- **Location Services**: Find nearby locations using GPS coordinates
- **Widgets**: Pre-built dropdown pickers for easy integration
- **Caching**: Smart caching system for performance and offline support
- **Streams**: Reactive programming support with Stream APIs