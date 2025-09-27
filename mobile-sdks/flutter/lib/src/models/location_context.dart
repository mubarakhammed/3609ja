import 'dart:math' as Math;
import 'postal_code.dart';

class LocationContext {
  final double latitude;
  final double longitude;
  final List<PostalCode> nearbyPostalCodes;
  final double? accuracy;
  final DateTime timestamp;

  LocationContext({
    required this.latitude,
    required this.longitude,
    required this.nearbyPostalCodes,
    this.accuracy,
    DateTime? timestamp,
  }) : timestamp = timestamp ?? DateTime.now();

  /// Get the closest postal code based on distance
  PostalCode? get closestPostalCode {
    if (nearbyPostalCodes.isEmpty) return null;

    // If postal codes have coordinates, find the closest one
    PostalCode? closest;
    double? minDistance;

    for (final postalCode in nearbyPostalCodes) {
      if (postalCode.latitude != null && postalCode.longitude != null) {
        final distance = _calculateDistance(
          latitude,
          longitude,
          postalCode.latitude!,
          postalCode.longitude!,
        );

        if (minDistance == null || distance < minDistance) {
          minDistance = distance;
          closest = postalCode;
        }
      }
    }

    // If no postal code has coordinates, return the first one
    return closest ?? nearbyPostalCodes.first;
  }

  /// Check if location has nearby postal codes
  bool get hasNearbyPostalCodes => nearbyPostalCodes.isNotEmpty;

  /// Get unique states from nearby postal codes
  Set<String> get nearbyStates => nearbyPostalCodes
      .where((pc) => pc.stateName != null)
      .map((pc) => pc.stateName!)
      .toSet();

  /// Get unique LGAs from nearby postal codes
  Set<String> get nearbyLGAs => nearbyPostalCodes
      .where((pc) => pc.lgaName != null)
      .map((pc) => pc.lgaName!)
      .toSet();

  /// Calculate distance between two coordinates in kilometers
  double _calculateDistance(
      double lat1, double lon1, double lat2, double lon2) {
    const double earthRadius = 6371.0; // Earth's radius in kilometers

    final double dLat = _toRadians(lat2 - lat1);
    final double dLon = _toRadians(lon2 - lon1);

    final double a = Math.sin(dLat / 2) * Math.sin(dLat / 2) +
        Math.cos(_toRadians(lat1)) *
            Math.cos(_toRadians(lat2)) *
            Math.sin(dLon / 2) *
            Math.sin(dLon / 2);

    final double c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));

    return earthRadius * c;
  }

  double _toRadians(double degrees) => degrees * Math.pi / 180.0;

  @override
  String toString() =>
      'LocationContext(lat: $latitude, lng: $longitude, nearby: ${nearbyPostalCodes.length})';
}
