class LGA {
  final String id;
  final String name;
  final String code;
  final String stateId;
  final String? stateName;
  final String? stateCode;
  final double? latitude;
  final double? longitude;
  final int? population;
  final double? area;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  const LGA({
    required this.id,
    required this.name,
    required this.code,
    required this.stateId,
    this.stateName,
    this.stateCode,
    this.latitude,
    this.longitude,
    this.population,
    this.area,
    this.createdAt,
    this.updatedAt,
  });

  factory LGA.fromJson(Map<String, dynamic> json) {
    return LGA(
      id: json['id'] as String,
      name: json['name'] as String,
      code: json['code'] as String,
      stateId: json['state_id'] as String,
      stateName: json['state_name'] as String?,
      stateCode: json['state_code'] as String?,
      latitude: (json['latitude'] as num?)?.toDouble(),
      longitude: (json['longitude'] as num?)?.toDouble(),
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
      'state_id': stateId,
      'state_name': stateName,
      'state_code': stateCode,
      'latitude': latitude,
      'longitude': longitude,
      'population': population,
      'area': area,
      'created_at': createdAt?.toIso8601String(),
      'updated_at': updatedAt?.toIso8601String(),
    };
  }

  @override
  String toString() =>
      'LGA(id: $id, name: $name, code: $code, stateId: $stateId)';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is LGA &&
        other.id == id &&
        other.name == name &&
        other.code == code &&
        other.stateId == stateId;
  }

  @override
  int get hashCode => Object.hash(id, name, code, stateId);

  LGA copyWith({
    String? id,
    String? name,
    String? code,
    String? stateId,
    String? stateName,
    String? stateCode,
    double? latitude,
    double? longitude,
    int? population,
    double? area,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return LGA(
      id: id ?? this.id,
      name: name ?? this.name,
      code: code ?? this.code,
      stateId: stateId ?? this.stateId,
      stateName: stateName ?? this.stateName,
      stateCode: stateCode ?? this.stateCode,
      latitude: latitude ?? this.latitude,
      longitude: longitude ?? this.longitude,
      population: population ?? this.population,
      area: area ?? this.area,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
