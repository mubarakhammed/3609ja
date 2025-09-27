import 'dart:async';

import 'package:geolocator/geolocator.dart';

import '../models/location_context.dart';
import '../exceptions/nigeria_geo_exception.dart';
import '../nigeria_geo_sdk.dart';
import '../utils/logger.dart';

/// Service for handling location-based operations
class LocationService {
  static LocationService? _instance;
  StreamSubscription<Position>? _positionSubscription;
  final StreamController<LocationContext> _locationStreamController =
      StreamController<LocationContext>.broadcast();

  LocationService._();

  /// Get singleton instance
  static LocationService get instance {
    _instance ??= LocationService._();
    return _instance!;
  }

  /// Check if location services are enabled
  static Future<bool> isLocationServiceEnabled() async {
    try {
      return await Geolocator.isLocationServiceEnabled();
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error checking location service status: $e');
      return false;
    }
  }

  /// Check current location permission status
  static Future<LocationPermission> getLocationPermission() async {
    try {
      return await Geolocator.checkPermission();
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error checking location permission: $e');
      return LocationPermission.denied;
    }
  }

  /// Request location permission
  static Future<LocationPermission> requestLocationPermission() async {
    try {
      final permission = await Geolocator.checkPermission();

      if (permission == LocationPermission.denied) {
        return await Geolocator.requestPermission();
      }

      return permission;
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error requesting location permission: $e');
      throw LocationException('Failed to request location permission: $e');
    }
  }

  /// Get current location with nearby postal codes
  static Future<LocationContext> getCurrentLocationContext({
    double radiusKm = 5.0,
    int maxResults = 20,
    LocationAccuracy accuracy = LocationAccuracy.high,
  }) async {
    try {
      // Check and request permissions
      await _ensureLocationPermission();

      // Get current position
      final position = await Geolocator.getCurrentPosition(
        desiredAccuracy: accuracy,
        timeLimit: const Duration(seconds: 30),
      );

      // Find nearby postal codes
      final nearbyPostalCodes =
          await NigeriaGeoSDK.client.findNearbyPostalCodes(
        latitude: position.latitude,
        longitude: position.longitude,
        radiusKm: radiusKm,
        limit: maxResults,
      );

      return LocationContext(
        latitude: position.latitude,
        longitude: position.longitude,
        nearbyPostalCodes: nearbyPostalCodes,
        accuracy: position.accuracy,
      );
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error getting current location context: $e');
      if (e is LocationException) rethrow;
      throw LocationException('Failed to get current location: $e');
    }
  }

  /// Start streaming location updates
  Stream<LocationContext> startLocationStream({
    double radiusKm = 5.0,
    int maxResults = 20,
    LocationAccuracy accuracy = LocationAccuracy.high,
    int distanceFilter = 100, // meters
    Duration interval = const Duration(seconds: 10),
  }) {
    _stopLocationStream(); // Stop any existing stream

    final locationSettings = LocationSettings(
      accuracy: accuracy,
      distanceFilter: distanceFilter,
      timeLimit: const Duration(seconds: 30),
    );

    _positionSubscription = Geolocator.getPositionStream(
      locationSettings: locationSettings,
    ).listen(
      (position) async {
        try {
          final nearbyPostalCodes =
              await NigeriaGeoSDK.client.findNearbyPostalCodes(
            latitude: position.latitude,
            longitude: position.longitude,
            radiusKm: radiusKm,
            limit: maxResults,
          );

          final locationContext = LocationContext(
            latitude: position.latitude,
            longitude: position.longitude,
            nearbyPostalCodes: nearbyPostalCodes,
            accuracy: position.accuracy,
          );

          if (!_locationStreamController.isClosed) {
            _locationStreamController.add(locationContext);
          }
        } catch (e) {
          NigeriaGeoLogger.instance.w('Error in location stream: $e');
          if (!_locationStreamController.isClosed) {
            _locationStreamController.addError(
              LocationException('Error updating location: $e'),
            );
          }
        }
      },
      onError: (error) {
        NigeriaGeoLogger.instance.e('Location stream error: $error');
        if (!_locationStreamController.isClosed) {
          _locationStreamController.addError(
            LocationException('Location stream error: $error'),
          );
        }
      },
    );

    return _locationStreamController.stream;
  }

  /// Stop location stream
  void stopLocationStream() {
    _stopLocationStream();
  }

  void _stopLocationStream() {
    _positionSubscription?.cancel();
    _positionSubscription = null;
  }

  /// Calculate distance between two points in kilometers
  static double calculateDistance(
    double lat1,
    double lon1,
    double lat2,
    double lon2,
  ) {
    return Geolocator.distanceBetween(lat1, lon1, lat2, lon2) / 1000.0;
  }

  /// Calculate bearing between two points
  static double calculateBearing(
    double lat1,
    double lon1,
    double lat2,
    double lon2,
  ) {
    return Geolocator.bearingBetween(lat1, lon1, lat2, lon2);
  }

  /// Get location accuracy description
  static String getAccuracyDescription(double? accuracy) {
    if (accuracy == null) return 'Unknown';

    if (accuracy <= 5) return 'Excellent';
    if (accuracy <= 10) return 'Good';
    if (accuracy <= 20) return 'Fair';
    if (accuracy <= 50) return 'Poor';
    return 'Very Poor';
  }

  /// Check if location permissions are granted
  static Future<bool> hasLocationPermission() async {
    final permission = await getLocationPermission();
    return permission == LocationPermission.always ||
        permission == LocationPermission.whileInUse;
  }

  /// Ensure location permission is granted
  static Future<void> _ensureLocationPermission() async {
    // Check if location service is enabled
    final serviceEnabled = await isLocationServiceEnabled();
    if (!serviceEnabled) {
      throw LocationException(
        'Location services are disabled. Please enable location services in your device settings.',
      );
    }

    // Check and request permission
    var permission = await getLocationPermission();

    if (permission == LocationPermission.denied) {
      permission = await requestLocationPermission();
    }

    if (permission == LocationPermission.denied) {
      throw LocationException('Location permission denied');
    }

    if (permission == LocationPermission.deniedForever) {
      throw LocationException(
        'Location permission permanently denied. Please grant location permission in app settings.',
      );
    }
  }

  /// Open device location settings
  static Future<bool> openLocationSettings() async {
    try {
      return await Geolocator.openLocationSettings();
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error opening location settings: $e');
      return false;
    }
  }

  /// Open app settings
  static Future<bool> openAppSettings() async {
    try {
      return await Geolocator.openAppSettings();
    } catch (e) {
      NigeriaGeoLogger.instance.e('Error opening app settings: $e');
      return false;
    }
  }

  /// Dispose location service
  void dispose() {
    _stopLocationStream();
    _locationStreamController.close();
    _instance = null;
  }
}
