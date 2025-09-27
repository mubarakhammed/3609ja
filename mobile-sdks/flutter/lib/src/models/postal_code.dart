class PostalCode {
  final String id;
  final String code;
  final String? area;
  final String? district;
  final String wardId;
  final String? wardName;
  final String? wardCode;
  final String? lgaId;
  final String? lgaName;
  final String? lgaCode;
  final String? stateId;
  final String? stateName;
  final String? stateCode;
  final double? latitude;
  final double? longitude;
  final String? deliveryZone;
  final bool? isActive;
  final DateTime? createdAt;
  final DateTime? updatedAt;

  const PostalCode({
    required this.id,
    required this.code,
    this.area,
    this.district,
    required this.wardId,
    this.wardName,
    this.wardCode,
    this.lgaId,
    this.lgaName,
    this.lgaCode,
    this.stateId,
    this.stateName,
    this.stateCode,
    this.latitude,
    this.longitude,
    this.deliveryZone,
    this.isActive,
    this.createdAt,
    this.updatedAt,
  });

  factory PostalCode.fromJson(Map<String, dynamic> json) {
    return PostalCode(
      id: json['id'] as String,
      code: json['code'] as String,
      area: json['area'] as String?,
      district: json['district'] as String?,
      wardId: json['ward_id'] as String,
      wardName: json['ward_name'] as String?,
      wardCode: json['ward_code'] as String?,
      lgaId: json['lga_id'] as String?,
      lgaName: json['lga_name'] as String?,
      lgaCode: json['lga_code'] as String?,
      stateId: json['state_id'] as String?,
      stateName: json['state_name'] as String?,
      stateCode: json['state_code'] as String?,
      latitude: (json['latitude'] as num?)?.toDouble(),
      longitude: (json['longitude'] as num?)?.toDouble(),
      deliveryZone: json['delivery_zone'] as String?,
      isActive: json['is_active'] as bool?,
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
      'code': code,
      'area': area,
      'district': district,
      'ward_id': wardId,
      'ward_name': wardName,
      'ward_code': wardCode,
      'lga_id': lgaId,
      'lga_name': lgaName,
      'lga_code': lgaCode,
      'state_id': stateId,
      'state_name': stateName,
      'state_code': stateCode,
      'latitude': latitude,
      'longitude': longitude,
      'delivery_zone': deliveryZone,
      'is_active': isActive,
      'created_at': createdAt?.toIso8601String(),
      'updated_at': updatedAt?.toIso8601String(),
    };
  }

  @override
  String toString() =>
      'PostalCode(id: $id, code: $code, area: $area, wardId: $wardId)';

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is PostalCode &&
        other.id == id &&
        other.code == code &&
        other.wardId == wardId;
  }

  @override
  int get hashCode => Object.hash(id, code, wardId);

  PostalCode copyWith({
    String? id,
    String? code,
    String? area,
    String? district,
    String? wardId,
    String? wardName,
    String? wardCode,
    String? lgaId,
    String? lgaName,
    String? lgaCode,
    String? stateId,
    String? stateName,
    String? stateCode,
    double? latitude,
    double? longitude,
    String? deliveryZone,
    bool? isActive,
    DateTime? createdAt,
    DateTime? updatedAt,
  }) {
    return PostalCode(
      id: id ?? this.id,
      code: code ?? this.code,
      area: area ?? this.area,
      district: district ?? this.district,
      wardId: wardId ?? this.wardId,
      wardName: wardName ?? this.wardName,
      wardCode: wardCode ?? this.wardCode,
      lgaId: lgaId ?? this.lgaId,
      lgaName: lgaName ?? this.lgaName,
      lgaCode: lgaCode ?? this.lgaCode,
      stateId: stateId ?? this.stateId,
      stateName: stateName ?? this.stateName,
      stateCode: stateCode ?? this.stateCode,
      latitude: latitude ?? this.latitude,
      longitude: longitude ?? this.longitude,
      deliveryZone: deliveryZone ?? this.deliveryZone,
      isActive: isActive ?? this.isActive,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
    );
  }
}
