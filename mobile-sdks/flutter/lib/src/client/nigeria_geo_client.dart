import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart' as http;
import 'package:logger/logger.dart';

import '../models/state.dart';
import '../models/lga.dart';
import '../models/ward.dart';
import '../models/postal_code.dart';
import '../models/search_result.dart';
import '../models/address_validation.dart';
import '../models/pagination.dart';
import '../exceptions/nigeria_geo_exception.dart';
import '../services/cache_service.dart';
import '../utils/logger.dart';
import 'nigeria_geo_config.dart';

class NigeriaGeoClient {
  final NigeriaGeoConfig _config;
  final http.Client _httpClient;
  final CacheService _cacheService;
  final Logger _logger;

  NigeriaGeoClient._(
    this._config,
    this._httpClient,
    this._cacheService,
  ) : _logger = NigeriaGeoLogger.instance;

  /// Internal factory constructor
  static NigeriaGeoClient create(
    NigeriaGeoConfig config,
    http.Client httpClient,
    CacheService cacheService,
  ) {
    return NigeriaGeoClient._(config, httpClient, cacheService);
  }

  // States API
  Future<PaginatedResponse<NigerianState>> getStates({
    int page = 1,
    int limit = 20,
    bool useCache = true,
  }) async {
    final cacheKey = 'states_${page}_$limit';

    if (useCache && _config.enableCaching) {
      final cached = await _cacheService.get<PaginatedResponse<NigerianState>>(
        cacheKey,
        (json) => PaginatedResponse.fromJson(
          json,
          (item) => NigerianState.fromJson(item as Map<String, dynamic>),
        ),
      );
      if (cached != null) {
        _logger.d('Returning cached states for page $page');
        return cached;
      }
    }

    try {
      final response = await _makeRequest(
        'GET',
        '/api/v1/states',
        queryParameters: {'page': page.toString(), 'limit': limit.toString()},
      );

      final result = PaginatedResponse<NigerianState>.fromJson(
        response,
        (item) => NigerianState.fromJson(item as Map<String, dynamic>),
      );

      if (_config.enableCaching) {
        await _cacheService.set(
            cacheKey, result.toJson((state) => state.toJson()));
      }

      return result;
    } catch (e) {
      _logger.e('Error fetching states: $e');
      rethrow;
    }
  }

  Future<NigerianState> getStateById(String stateId) async {
    final cacheKey = 'state_$stateId';

    if (_config.enableCaching) {
      final cached = await _cacheService.get<NigerianState>(
        cacheKey,
        (json) => NigerianState.fromJson(json),
      );
      if (cached != null) return cached;
    }

    try {
      final response = await _makeRequest('GET', '/api/v1/states/$stateId');
      final state = NigerianState.fromJson(response['data']);

      if (_config.enableCaching) {
        await _cacheService.set(cacheKey, state.toJson());
      }

      return state;
    } catch (e) {
      _logger.e('Error fetching state $stateId: $e');
      rethrow;
    }
  }

  // LGAs API
  Future<PaginatedResponse<LGA>> getLGAs({
    String? stateId,
    int page = 1,
    int limit = 20,
    bool useCache = true,
  }) async {
    final cacheKey = 'lgas_${stateId ?? 'all'}_${page}_$limit';

    if (useCache && _config.enableCaching) {
      final cached = await _cacheService.get<PaginatedResponse<LGA>>(
        cacheKey,
        (json) => PaginatedResponse.fromJson(
          json,
          (item) => LGA.fromJson(item as Map<String, dynamic>),
        ),
      );
      if (cached != null) return cached;
    }

    try {
      final queryParams = <String, String>{
        'page': page.toString(),
        'limit': limit.toString(),
      };

      final String endpoint;
      if (stateId != null) {
        endpoint = '/api/v1/states/$stateId/lgas';
      } else {
        throw ArgumentError('stateId is required for fetching LGAs');
      }

      final response =
          await _makeRequest('GET', endpoint, queryParameters: queryParams);

      final result = PaginatedResponse<LGA>.fromJson(
        response,
        (item) => LGA.fromJson(item as Map<String, dynamic>),
      );

      if (_config.enableCaching) {
        await _cacheService.set(cacheKey, result.toJson((lga) => lga.toJson()));
      }

      return result;
    } catch (e) {
      _logger.e('Error fetching LGAs: $e');
      rethrow;
    }
  }

  Future<LGA> getLGAById(String lgaId) async {
    final cacheKey = 'lga_$lgaId';

    if (_config.enableCaching) {
      final cached = await _cacheService.get<LGA>(
        cacheKey,
        (json) => LGA.fromJson(json),
      );
      if (cached != null) return cached;
    }

    try {
      final response = await _makeRequest('GET', '/api/v1/lgas/$lgaId');
      final lga = LGA.fromJson(response['data']);

      if (_config.enableCaching) {
        await _cacheService.set(cacheKey, lga.toJson());
      }

      return lga;
    } catch (e) {
      _logger.e('Error fetching LGA $lgaId: $e');
      rethrow;
    }
  }

  // Wards API
  Future<PaginatedResponse<Ward>> getWards({
    String? lgaId,
    int page = 1,
    int limit = 20,
    bool useCache = true,
  }) async {
    final cacheKey = 'wards_${lgaId ?? 'all'}_${page}_$limit';

    if (useCache && _config.enableCaching) {
      final cached = await _cacheService.get<PaginatedResponse<Ward>>(
        cacheKey,
        (json) => PaginatedResponse.fromJson(
          json,
          (item) => Ward.fromJson(item as Map<String, dynamic>),
        ),
      );
      if (cached != null) return cached;
    }

    try {
      final queryParams = <String, String>{
        'page': page.toString(),
        'limit': limit.toString(),
      };

      final String endpoint;
      if (lgaId != null) {
        endpoint = '/api/v1/lgas/$lgaId/wards';
      } else {
        throw ArgumentError('lgaId is required for fetching wards');
      }

      final response =
          await _makeRequest('GET', endpoint, queryParameters: queryParams);

      final result = PaginatedResponse<Ward>.fromJson(
        response,
        (item) => Ward.fromJson(item as Map<String, dynamic>),
      );

      if (_config.enableCaching) {
        await _cacheService.set(
            cacheKey, result.toJson((ward) => ward.toJson()));
      }

      return result;
    } catch (e) {
      _logger.e('Error fetching wards: $e');
      rethrow;
    }
  }

  // Postal Codes API
  Future<PaginatedResponse<PostalCode>> getPostalCodes({
    String? wardId,
    int page = 1,
    int limit = 20,
    bool useCache = true,
  }) async {
    final cacheKey = 'postal_codes_${wardId ?? 'all'}_${page}_$limit';

    if (useCache && _config.enableCaching) {
      final cached = await _cacheService.get<PaginatedResponse<PostalCode>>(
        cacheKey,
        (json) => PaginatedResponse.fromJson(
          json,
          (item) => PostalCode.fromJson(item as Map<String, dynamic>),
        ),
      );
      if (cached != null) return cached;
    }

    try {
      final queryParams = <String, String>{
        'page': page.toString(),
        'limit': limit.toString(),
      };

      final String endpoint;
      if (wardId != null) {
        endpoint = '/api/v1/wards/$wardId/postal-codes';
      } else {
        throw ArgumentError('wardId is required for fetching postal codes');
      }

      final response =
          await _makeRequest('GET', endpoint, queryParameters: queryParams);

      final result = PaginatedResponse<PostalCode>.fromJson(
        response,
        (item) => PostalCode.fromJson(item as Map<String, dynamic>),
      );

      if (_config.enableCaching) {
        await _cacheService.set(
            cacheKey, result.toJson((postalCode) => postalCode.toJson()));
      }

      return result;
    } catch (e) {
      _logger.e('Error fetching postal codes: $e');
      rethrow;
    }
  }

  // Search API
  Future<SearchResult> searchAll(String query, {bool useCache = true}) async {
    if (query.trim().isEmpty) {
      return SearchResult(states: [], lgas: [], wards: [], postalCodes: []);
    }

    final cacheKey = 'search_${query.toLowerCase().trim()}';

    if (useCache && _config.enableCaching) {
      final cached = await _cacheService.get<SearchResult>(
        cacheKey,
        (json) => SearchResult.fromJson(json),
      );
      if (cached != null) return cached;
    }

    try {
      final response = await _makeRequest(
        'GET',
        '/api/v1/search',
        queryParameters: {'query': query.trim()},
      );

      final result = SearchResult.fromJson(response);

      if (_config.enableCaching) {
        await _cacheService.set(cacheKey, result.toJson());
      }

      return result;
    } catch (e) {
      _logger.e('Error searching for "$query": $e');
      rethrow;
    }
  }

  // Address Validation API
  Future<AddressValidationResponse> validateAddress(
    AddressValidationRequest request,
  ) async {
    try {
      final response = await _makeRequest(
        'POST',
        '/api/v1/validate',
        body: request.toJson(),
      );

      return AddressValidationResponse.fromJson(response['data']);
    } catch (e) {
      _logger.e('Error validating address: $e');
      rethrow;
    }
  }

  // Location-based queries
  Future<List<PostalCode>> findNearbyPostalCodes({
    required double latitude,
    required double longitude,
    double radiusKm = 5.0,
    int limit = 20,
  }) async {
    try {
      final response = await _makeRequest(
        'GET',
        '/api/v1/postal-codes/nearby',
        queryParameters: {
          'lat': latitude.toString(),
          'lng': longitude.toString(),
          'radius': radiusKm.toString(),
          'limit': limit.toString(),
        },
      );

      final List<dynamic> items = response['data'] ?? [];
      return items.map((item) => PostalCode.fromJson(item)).toList();
    } catch (e) {
      _logger.e('Error finding nearby postal codes: $e');
      rethrow;
    }
  }

  // Stream APIs
  Stream<List<NigerianState>> getStatesStream() {
    return Stream.fromFuture(getStates().then((response) => response.data));
  }

  Stream<List<LGA>> getLGAsStream({String? stateId}) {
    return Stream.fromFuture(
        getLGAs(stateId: stateId).then((response) => response.data));
  }

  Stream<List<Ward>> getWardsStream({String? lgaId}) {
    return Stream.fromFuture(
        getWards(lgaId: lgaId).then((response) => response.data));
  }

  // Private helper methods
  Future<Map<String, dynamic>> _makeRequest(
    String method,
    String endpoint, {
    Map<String, String>? queryParameters,
    Map<String, dynamic>? body,
    Map<String, String>? headers,
  }) async {
    final uri = Uri.parse('${_config.baseUrl}$endpoint')
        .replace(queryParameters: queryParameters);

    final requestHeaders = <String, String>{
      'Content-Type': 'application/json',
      'Accept': 'application/json',
      ...?headers,
    };

    if (_config.apiKey != null && _config.apiKey!.isNotEmpty) {
      requestHeaders['Authorization'] = 'Bearer ${_config.apiKey}';
    }

    if (_config.enableLogging) {
      _logger.d('$method $uri');
      if (body != null) _logger.d('Body: $body');
    }

    try {
      late http.Response response;

      switch (method.toUpperCase()) {
        case 'GET':
          response = await _httpClient
              .get(uri, headers: requestHeaders)
              .timeout(_config.timeout);
          break;
        case 'POST':
          response = await _httpClient
              .post(uri,
                  headers: requestHeaders,
                  body: body != null ? jsonEncode(body) : null)
              .timeout(_config.timeout);
          break;
        case 'PUT':
          response = await _httpClient
              .put(uri,
                  headers: requestHeaders,
                  body: body != null ? jsonEncode(body) : null)
              .timeout(_config.timeout);
          break;
        case 'DELETE':
          response = await _httpClient
              .delete(uri, headers: requestHeaders)
              .timeout(_config.timeout);
          break;
        default:
          throw UnsupportedError('HTTP method $method not supported');
      }

      if (_config.enableLogging) {
        _logger.d('Response: ${response.statusCode} ${response.body}');
      }

      return _handleResponse(response);
    } on TimeoutException {
      throw NetworkException(
          'Request timeout after ${_config.timeout.inSeconds}s');
    } on SocketException catch (e) {
      throw NetworkException('Network error: ${e.message}');
    } on http.ClientException catch (e) {
      throw NetworkException('HTTP client error: ${e.message}');
    } catch (e) {
      _logger.e('Unexpected error in _makeRequest: $e');
      rethrow;
    }
  }

  Map<String, dynamic> _handleResponse(http.Response response) {
    final statusCode = response.statusCode;

    try {
      final jsonResponse = jsonDecode(response.body) as Map<String, dynamic>;

      if (statusCode >= 200 && statusCode < 300) {
        return jsonResponse;
      } else {
        final message =
            jsonResponse['message'] ?? jsonResponse['error'] ?? 'Unknown error';

        switch (statusCode) {
          case 400:
            throw InvalidRequestException(message);
          case 401:
            throw UnauthorizedException(message);
          case 403:
            throw ForbiddenException(message);
          case 404:
            throw NotFoundException(message);
          case 429:
            throw RateLimitException(message);
          case 500:
            throw ServerException(message);
          default:
            throw ServerException('HTTP $statusCode: $message');
        }
      }
    } catch (FormatException) {
      throw InvalidResponseException('Invalid JSON response');
    }
  }

  void dispose() {
    _httpClient.close();
  }
}
