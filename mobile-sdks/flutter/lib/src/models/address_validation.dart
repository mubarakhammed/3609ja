class AddressValidationRequest {
  final String state;
  final String lga;
  final String? ward;
  final String? postalCode;
  final String? streetAddress;
  final String? city;
  final bool? strictValidation;

  const AddressValidationRequest({
    required this.state,
    required this.lga,
    this.ward,
    this.postalCode,
    this.streetAddress,
    this.city,
    this.strictValidation,
  });

  factory AddressValidationRequest.fromJson(Map<String, dynamic> json) {
    return AddressValidationRequest(
      state: json['state'] as String,
      lga: json['lga'] as String,
      ward: json['ward'] as String?,
      postalCode: json['postal_code'] as String?,
      streetAddress: json['street_address'] as String?,
      city: json['city'] as String?,
      strictValidation: json['strict_validation'] as bool?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'state': state,
      'lga': lga,
      'ward': ward,
      'postal_code': postalCode,
      'street_address': streetAddress,
      'city': city,
      'strict_validation': strictValidation,
    };
  }

  @override
  String toString() =>
      'AddressValidationRequest(state: $state, lga: $lga, ward: $ward, postalCode: $postalCode)';
}

class AddressValidationResponse {
  final bool isValid;
  final double confidence;
  final NormalizedAddress normalizedAddress;
  final List<AddressSuggestion> suggestions;
  final List<ValidationError> errors;
  final Map<String, dynamic>? metadata;

  const AddressValidationResponse({
    required this.isValid,
    required this.confidence,
    required this.normalizedAddress,
    required this.suggestions,
    required this.errors,
    this.metadata,
  });

  factory AddressValidationResponse.fromJson(Map<String, dynamic> json) {
    return AddressValidationResponse(
      isValid: json['is_valid'] as bool,
      confidence: (json['confidence'] as num).toDouble(),
      normalizedAddress: NormalizedAddress.fromJson(
          json['normalized_address'] as Map<String, dynamic>),
      suggestions: (json['suggestions'] as List<dynamic>? ?? [])
          .map((item) =>
              AddressSuggestion.fromJson(item as Map<String, dynamic>))
          .toList(),
      errors: (json['errors'] as List<dynamic>? ?? [])
          .map((item) => ValidationError.fromJson(item as Map<String, dynamic>))
          .toList(),
      metadata: json['metadata'] as Map<String, dynamic>?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'is_valid': isValid,
      'confidence': confidence,
      'normalized_address': normalizedAddress.toJson(),
      'suggestions': suggestions.map((s) => s.toJson()).toList(),
      'errors': errors.map((e) => e.toJson()).toList(),
      'metadata': metadata,
    };
  }

  /// Check if validation has warnings (not errors)
  bool get hasWarnings => confidence < 1.0 && isValid;

  /// Check if validation has critical errors
  bool get hasCriticalErrors =>
      errors.any((e) => e.severity == ErrorSeverity.error);

  @override
  String toString() =>
      'AddressValidationResponse(isValid: $isValid, confidence: $confidence, errors: ${errors.length})';
}

class NormalizedAddress {
  final String state;
  final String stateCode;
  final String lga;
  final String lgaCode;
  final String? ward;
  final String? wardCode;
  final String? postalCode;
  final String? streetAddress;
  final String? city;
  final double? latitude;
  final double? longitude;
  final String? formattedAddress;

  const NormalizedAddress({
    required this.state,
    required this.stateCode,
    required this.lga,
    required this.lgaCode,
    this.ward,
    this.wardCode,
    this.postalCode,
    this.streetAddress,
    this.city,
    this.latitude,
    this.longitude,
    this.formattedAddress,
  });

  factory NormalizedAddress.fromJson(Map<String, dynamic> json) {
    return NormalizedAddress(
      state: json['state'] as String,
      stateCode: json['state_code'] as String,
      lga: json['lga'] as String,
      lgaCode: json['lga_code'] as String,
      ward: json['ward'] as String?,
      wardCode: json['ward_code'] as String?,
      postalCode: json['postal_code'] as String?,
      streetAddress: json['street_address'] as String?,
      city: json['city'] as String?,
      latitude: (json['latitude'] as num?)?.toDouble(),
      longitude: (json['longitude'] as num?)?.toDouble(),
      formattedAddress: json['formatted_address'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'state': state,
      'state_code': stateCode,
      'lga': lga,
      'lga_code': lgaCode,
      'ward': ward,
      'ward_code': wardCode,
      'postal_code': postalCode,
      'street_address': streetAddress,
      'city': city,
      'latitude': latitude,
      'longitude': longitude,
      'formatted_address': formattedAddress,
    };
  }

  @override
  String toString() =>
      formattedAddress ?? '$streetAddress, $ward, $lga, $state';
}

class AddressSuggestion {
  final String title;
  final String description;
  final NormalizedAddress address;
  final double confidence;
  final String? reason;

  const AddressSuggestion({
    required this.title,
    required this.description,
    required this.address,
    required this.confidence,
    this.reason,
  });

  factory AddressSuggestion.fromJson(Map<String, dynamic> json) {
    return AddressSuggestion(
      title: json['title'] as String,
      description: json['description'] as String,
      address:
          NormalizedAddress.fromJson(json['address'] as Map<String, dynamic>),
      confidence: (json['confidence'] as num).toDouble(),
      reason: json['reason'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'title': title,
      'description': description,
      'address': address.toJson(),
      'confidence': confidence,
      'reason': reason,
    };
  }

  @override
  String toString() =>
      'AddressSuggestion(title: $title, confidence: $confidence)';
}

class ValidationError {
  final String field;
  final String message;
  final ErrorSeverity severity;
  final String? code;
  final Map<String, dynamic>? context;

  const ValidationError({
    required this.field,
    required this.message,
    required this.severity,
    this.code,
    this.context,
  });

  factory ValidationError.fromJson(Map<String, dynamic> json) {
    return ValidationError(
      field: json['field'] as String,
      message: json['message'] as String,
      severity: ErrorSeverity.values.firstWhere(
        (e) => e.toString().split('.').last == json['severity'],
        orElse: () => ErrorSeverity.error,
      ),
      code: json['code'] as String?,
      context: json['context'] as Map<String, dynamic>?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'field': field,
      'message': message,
      'severity': severity.toString().split('.').last,
      'code': code,
      'context': context,
    };
  }

  @override
  String toString() =>
      'ValidationError(field: $field, message: $message, severity: $severity)';
}

enum ErrorSeverity {
  info,
  warning,
  error,
}
