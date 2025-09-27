import 'package:flutter_test/flutter_test.dart';
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

void main() {
  group('Nigeria Geo SDK Tests', () {
    setUpAll(() async {
      // Initialize SDK for testing
      await NigeriaGeoSDK.initialize(
        NigeriaGeoConfig.development(
          baseUrl: 'https://test-api.example.com',
        ),
      );
    });

    tearDownAll(() async {
      await NigeriaGeoSDK.dispose();
    });

    test('SDK initialization', () {
      expect(NigeriaGeoSDK.isInitialized, true);
      expect(NigeriaGeoSDK.client, isNotNull);
      expect(NigeriaGeoSDK.config, isNotNull);
      expect(NigeriaGeoSDK.cacheManager, isNotNull);
    });

    test('NigerianState model', () {
      final state = NigerianState(
        id: '1',
        name: 'Lagos',
        code: 'LA',
        capital: 'Ikeja',
      );

      expect(state.id, '1');
      expect(state.name, 'Lagos');
      expect(state.code, 'LA');
      expect(state.capital, 'Ikeja');

      // Test JSON serialization
      final json = state.toJson();
      expect(json['id'], '1');
      expect(json['name'], 'Lagos');
      expect(json['code'], 'LA');
      expect(json['capital'], 'Ikeja');

      // Test JSON deserialization
      final stateFromJson = NigerianState.fromJson(json);
      expect(stateFromJson.id, state.id);
      expect(stateFromJson.name, state.name);
      expect(stateFromJson.code, state.code);
      expect(stateFromJson.capital, state.capital);
    });

    test('SearchResult model', () {
      final state = NigerianState(id: '1', name: 'Lagos', code: 'LA');
      final lga = LGA(id: '1', name: 'Ikeja', code: 'IK', stateId: '1');

      final searchResult = SearchResult(
        states: [state],
        lgas: [lga],
        wards: [],
        postalCodes: [],
      );

      expect(searchResult.totalResults, 2);
      expect(searchResult.hasResults, true);
      expect(searchResult.isEmpty, false);
      expect(searchResult.states.length, 1);
      expect(searchResult.lgas.length, 1);
    });

    test('SearchResultItem creation', () {
      final state =
          NigerianState(id: '1', name: 'Lagos', code: 'LA', capital: 'Ikeja');
      final item = SearchResultItem(type: SearchType.state, data: state);

      expect(item.type, SearchType.state);
      expect(item.title, 'Lagos');
      expect(item.subtitle, 'State • Capital: Ikeja');
      expect(item.id, '1');
    });

    test('Configuration', () {
      final config = NigeriaGeoConfig.development(
        baseUrl: 'https://test.example.com',
        apiKey: 'test-key',
      );

      expect(config.baseUrl, 'https://test.example.com');
      expect(config.apiKey, 'test-key');
      expect(config.enableCaching, true);
      expect(config.enableLogging, true);
      expect(config.maxRetries, 1);

      final prodConfig = NigeriaGeoConfig.production(
        baseUrl: 'https://api.example.com',
        apiKey: 'prod-key',
      );

      expect(prodConfig.enableLogging, false);
      expect(prodConfig.maxRetries, 3);
    });

    test('Exception types', () {
      const networkException = NetworkException('Network error');
      expect(networkException.message, 'Network error');
      expect(networkException.code, 'NETWORK_ERROR');

      const notFoundException = NotFoundException('Not found');
      expect(notFoundException.message, 'Not found');
      expect(notFoundException.code, 'NOT_FOUND');

      const rateLimitException = RateLimitException(
        'Rate limited',
        retryAfterSeconds: 30,
      );
      expect(rateLimitException.retryAfterSeconds, 30);
    });

    test('Cache statistics', () {
      final stats = CacheStatistics(
        itemCount: 100,
        totalSize: 1024 * 1024, // 1MB
        hitCount: 80,
        missCount: 20,
      );

      expect(stats.itemCount, 100);
      expect(stats.sizeInMB, 1.0);
      expect(stats.hitRate, 0.8);
    });
  });
}
