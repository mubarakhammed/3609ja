import 'state.dart';
import 'lga.dart';
import 'ward.dart';
import 'postal_code.dart';

class SearchResult {
  final List<NigerianState> states;
  final List<LGA> lgas;
  final List<Ward> wards;
  final List<PostalCode> postalCodes;

  const SearchResult({
    required this.states,
    required this.lgas,
    required this.wards,
    required this.postalCodes,
  });

  factory SearchResult.fromJson(Map<String, dynamic> json) {
    return SearchResult(
      states: (json['states'] as List<dynamic>? ?? [])
          .map((item) => NigerianState.fromJson(item as Map<String, dynamic>))
          .toList(),
      lgas: (json['lgas'] as List<dynamic>? ?? [])
          .map((item) => LGA.fromJson(item as Map<String, dynamic>))
          .toList(),
      wards: (json['wards'] as List<dynamic>? ?? [])
          .map((item) => Ward.fromJson(item as Map<String, dynamic>))
          .toList(),
      postalCodes: (json['postal_codes'] as List<dynamic>? ?? [])
          .map((item) => PostalCode.fromJson(item as Map<String, dynamic>))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'states': states.map((s) => s.toJson()).toList(),
      'lgas': lgas.map((l) => l.toJson()).toList(),
      'wards': wards.map((w) => w.toJson()).toList(),
      'postal_codes': postalCodes.map((p) => p.toJson()).toList(),
    };
  }

  /// Get total number of results across all categories
  int get totalResults =>
      states.length + lgas.length + wards.length + postalCodes.length;

  /// Check if search returned any results
  bool get hasResults => totalResults > 0;

  /// Check if search has no results
  bool get isEmpty => totalResults == 0;

  @override
  String toString() =>
      'SearchResult(states: ${states.length}, lgas: ${lgas.length}, '
      'wards: ${wards.length}, postalCodes: ${postalCodes.length})';
}

/// Unified search result item for UI display
class SearchResultItem {
  final SearchType type;
  final dynamic data;

  SearchResultItem({
    required this.type,
    required this.data,
  });

  String get title {
    switch (type) {
      case SearchType.state:
        return (data as NigerianState).name;
      case SearchType.lga:
        return (data as LGA).name;
      case SearchType.ward:
        return (data as Ward).name;
      case SearchType.postalCode:
        return (data as PostalCode).code;
    }
  }

  String get subtitle {
    switch (type) {
      case SearchType.state:
        final state = data as NigerianState;
        return 'State • Capital: ${state.capital ?? 'N/A'}';
      case SearchType.lga:
        final lga = data as LGA;
        return 'LGA • ${lga.stateName ?? 'Unknown State'}';
      case SearchType.ward:
        final ward = data as Ward;
        return 'Ward • ${ward.lgaName ?? 'Unknown LGA'}';
      case SearchType.postalCode:
        final postal = data as PostalCode;
        return 'Postal Code • ${postal.area ?? postal.wardName ?? 'Unknown Area'}';
    }
  }

  String get id {
    switch (type) {
      case SearchType.state:
        return (data as NigerianState).id;
      case SearchType.lga:
        return (data as LGA).id;
      case SearchType.ward:
        return (data as Ward).id;
      case SearchType.postalCode:
        return (data as PostalCode).id;
    }
  }

  @override
  String toString() => 'SearchResultItem(type: $type, title: $title)';
}

enum SearchType {
  state,
  lga,
  ward,
  postalCode,
}
