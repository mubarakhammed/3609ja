/// Base exception class for all Nigeria Geo SDK exceptions
abstract class NigeriaGeoException implements Exception {
  final String message;
  final String? code;
  final dynamic originalError;
  final StackTrace? stackTrace;

  const NigeriaGeoException(
    this.message, {
    this.code,
    this.originalError,
    this.stackTrace,
  });

  @override
  String toString() {
    final buffer = StringBuffer('$runtimeType: $message');
    if (code != null) {
      buffer.write(' (Code: $code)');
    }
    return buffer.toString();
  }
}

/// Network-related exceptions
class NetworkException extends NigeriaGeoException {
  const NetworkException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'NETWORK_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Invalid response format exceptions
class InvalidResponseException extends NigeriaGeoException {
  const InvalidResponseException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'INVALID_RESPONSE',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Authentication/authorization exceptions
class UnauthorizedException extends NigeriaGeoException {
  const UnauthorizedException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'UNAUTHORIZED',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Forbidden access exceptions
class ForbiddenException extends NigeriaGeoException {
  const ForbiddenException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'FORBIDDEN',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Resource not found exceptions
class NotFoundException extends NigeriaGeoException {
  const NotFoundException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'NOT_FOUND',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Rate limiting exceptions
class RateLimitException extends NigeriaGeoException {
  final int? retryAfterSeconds;

  const RateLimitException(
    String message, {
    String? code,
    this.retryAfterSeconds,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'RATE_LIMIT',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Server error exceptions
class ServerException extends NigeriaGeoException {
  const ServerException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'SERVER_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Invalid request exceptions
class InvalidRequestException extends NigeriaGeoException {
  const InvalidRequestException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'INVALID_REQUEST',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Cache-related exceptions
class CacheException extends NigeriaGeoException {
  const CacheException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'CACHE_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Location service exceptions
class LocationException extends NigeriaGeoException {
  const LocationException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'LOCATION_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// Configuration exceptions
class ConfigurationException extends NigeriaGeoException {
  const ConfigurationException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'CONFIGURATION_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}

/// SDK initialization exceptions
class InitializationException extends NigeriaGeoException {
  const InitializationException(
    String message, {
    String? code,
    dynamic originalError,
    StackTrace? stackTrace,
  }) : super(
          message,
          code: code ?? 'INITIALIZATION_ERROR',
          originalError: originalError,
          stackTrace: stackTrace,
        );
}
