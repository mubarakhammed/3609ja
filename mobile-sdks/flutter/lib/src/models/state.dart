class NigerianState {
  final String id;
  final String name;
  final String code;
  final String? capital;
  final double? latitude;
  final double? longitude;
  final String? region;
  final int? population;
  final double? area;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  const NigerianState({
    required this.id,
    required this.name,
    required this.code,
    this.capital,
    this.latitude,
    this.longitude,
    this.region,
    this.population,
    this.area,
    this.createdAt,
    this.updatedAt,
  });

  factory NigerianState.fromJson(Map<String, dynamic> json) {
    return NigerianState(
      id: json['id'] as String,
      name: json['name'] as String,
      code: json['code'] as String,
      capital: json['capital'] as String?,
      latitude: (json['latitude'] as num?)?.toDouble(),
      longitude: (json['longitude'] as num?)?.toDouble(),
      region: json['region'] as String?,
      population: json['population'] as int?,
      area: (json['area'] as num?)?.toDouble(),
      createdAt: json['created_at'] != null
          ? DateTime.parse(json['created_at'] as String)
          : null,
      updatedAt: json['updated_at'] != null
          ? DateTime.parse(json['updated_at'] as String)
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'name': name,
      'code': code,
      'capital': capital,
      'latitude': latitude,
      'longitude': longitude,
      'region': region,
      'population': population,
      'area': area,
      'created_at': createdAt?.toIso8601String(),
      'updated_at': updatedAt?.toIso8601String(),
    };
  }

  @override
  String toString() => 'NigerianState(id: $id, name: $name, code: $code)';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is NigerianState &&
        other.id == id &&
        other.name == name &&
        other.code == code;
  }

  @override
  int get hashCode => Object.hash(id, name, code);

  /// Copy with modified values
  NigerianState copyWith({
    String? id,
    String? name,
    String? code,
    String? capital,
    double? latitude,
    double? longitude,
    String? region,
    int? population,
    double? area,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return NigerianState(
      id: id ?? this.id,
      name: name ?? this.name,
      code: code ?? this.code,
      capital: capital ?? this.capital,
      latitude: latitude ?? this.latitude,
      longitude: longitude ?? this.longitude,
      region: region ?? this.region,
      population: population ?? this.population,
      area: area ?? this.area,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
