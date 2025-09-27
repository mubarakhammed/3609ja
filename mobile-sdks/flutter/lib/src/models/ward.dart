class Ward {
  final String id;
  final String name;
  final String code;
  final String lgaId;
  final String? lgaName;
  final String? lgaCode;
  final String? stateId;
  final String? stateName;
  final String? stateCode;
  final double? latitude;
  final double? longitude;
  final int? population;
  final double? area;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  const Ward({
    required this.id,
    required this.name,
    required this.code,
    required this.lgaId,
    this.lgaName,
    this.lgaCode,
    this.stateId,
    this.stateName,
    this.stateCode,
    this.latitude,
    this.longitude,
    this.population,
    this.area,
    this.createdAt,
    this.updatedAt,
  });

  factory Ward.fromJson(Map<String, dynamic> json) {
    return Ward(
      id: json['id'] as String,
      name: json['name'] as String,
      code: json['code'] as String,
      lgaId: json['lga_id'] as String,
      lgaName: json['lga_name'] as String?,
      lgaCode: json['lga_code'] as String?,
      stateId: json['state_id'] as String?,
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
      'lga_id': lgaId,
      'lga_name': lgaName,
      'lga_code': lgaCode,
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
  String toString() => 'Ward(id: $id, name: $name, code: $code, lgaId: $lgaId)';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is Ward &&
        other.id == id &&
        other.name == name &&
        other.code == code &&
        other.lgaId == lgaId;
  }

  @override
  int get hashCode => Object.hash(id, name, code, lgaId);

  Ward copyWith({
    String? id,
    String? name,
    String? code,
    String? lgaId,
    String? lgaName,
    String? lgaCode,
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
    return Ward(
      id: id ?? this.id,
      name: name ?? this.name,
      code: code ?? this.code,
      lgaId: lgaId ?? this.lgaId,
      lgaName: lgaName ?? this.lgaName,
      lgaCode: lgaCode ?? this.lgaCode,
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
