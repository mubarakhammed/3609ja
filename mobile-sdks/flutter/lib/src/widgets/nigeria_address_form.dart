import 'package:flutter/material.dart';
import '../client/nigeria_geo_client.dart';
import '../models/state.dart';
import '../models/lga.dart';
import '../models/ward.dart';
import 'nigeria_state_picker.dart';
import 'nigeria_lga_picker.dart';
import 'nigeria_ward_picker.dart';

/// A complete address form widget for Nigerian addresses
class NigeriaAddressForm extends StatefulWidget {
  /// The Nigeria Geo Client instance
  final NigeriaGeoClient client;

  /// Callback when the address changes
  final void Function(NigeriaAddressData) onAddressChanged;

  /// Initial address data
  final NigeriaAddressData? initialAddress;

  /// Whether to show postal code field
  final bool showPostalCode;

  /// Whether to show street address field
  final bool showStreetAddress;

  /// Whether the form is enabled
  final bool enabled;

  const NigeriaAddressForm({
    super.key,
    required this.client,
    required this.onAddressChanged,
    this.initialAddress,
    this.showPostalCode = true,
    this.showStreetAddress = true,
    this.enabled = true,
  });

  @override
  State<NigeriaAddressForm> createState() => _NigeriaAddressFormState();
}

class _NigeriaAddressFormState extends State<NigeriaAddressForm> {
  final _formKey = GlobalKey<FormState>();
  final _streetController = TextEditingController();
  final _postalCodeController = TextEditingController();

  NigerianState? _selectedState;
  LGA? _selectedLGA;
  Ward? _selectedWard;

  @override
  void initState() {
    super.initState();
    if (widget.initialAddress != null) {
      _streetController.text = widget.initialAddress!.streetAddress ?? '';
      _postalCodeController.text = widget.initialAddress!.postalCode ?? '';
      _selectedState = widget.initialAddress!.state;
      _selectedLGA = widget.initialAddress!.lga;
      _selectedWard = widget.initialAddress!.ward;
    }
  }

  @override
  void dispose() {
    _streetController.dispose();
    _postalCodeController.dispose();
    super.dispose();
  }

  void _onAddressFieldChanged() {
    final address = NigeriaAddressData(
      streetAddress:
          widget.showStreetAddress ? _streetController.text.trim() : null,
      state: _selectedState,
      lga: _selectedLGA,
      ward: _selectedWard,
      postalCode:
          widget.showPostalCode ? _postalCodeController.text.trim() : null,
    );

    widget.onAddressChanged(address);
  }

  void _onStateSelected(NigerianState state) {
    setState(() {
      _selectedState = state;
      _selectedLGA = null; // Reset dependent fields
      _selectedWard = null;
    });
    _onAddressFieldChanged();
  }

  void _onLGASelected(LGA lga) {
    setState(() {
      _selectedLGA = lga;
      _selectedWard = null; // Reset dependent field
    });
    _onAddressFieldChanged();
  }

  void _onWardSelected(Ward ward) {
    setState(() {
      _selectedWard = ward;
    });
    _onAddressFieldChanged();
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (widget.showStreetAddress) ...[
            TextFormField(
              controller: _streetController,
              decoration: const InputDecoration(
                labelText: 'Street Address',
                hintText: 'Enter street address',
                border: OutlineInputBorder(),
              ),
              enabled: widget.enabled,
              onChanged: (_) => _onAddressFieldChanged(),
              validator: (value) {
                if (value == null || value.trim().isEmpty) {
                  return 'Please enter a street address';
                }
                return null;
              },
            ),
            const SizedBox(height: 16),
          ],
          NigeriaStatePicker(
            client: widget.client,
            selectedState: _selectedState,
            onStateSelected: _onStateSelected,
            enabled: widget.enabled,
          ),
          const SizedBox(height: 16),
          NigeriaLGAPicker(
            client: widget.client,
            state: _selectedState,
            selectedLGA: _selectedLGA,
            onLGASelected: _onLGASelected,
            enabled: widget.enabled,
          ),
          const SizedBox(height: 16),
          NigeriaWardPicker(
            client: widget.client,
            lga: _selectedLGA,
            selectedWard: _selectedWard,
            onWardSelected: _onWardSelected,
            enabled: widget.enabled,
          ),
          if (widget.showPostalCode) ...[
            const SizedBox(height: 16),
            TextFormField(
              controller: _postalCodeController,
              decoration: const InputDecoration(
                labelText: 'Postal Code',
                hintText: 'Enter postal code',
                border: OutlineInputBorder(),
              ),
              enabled: widget.enabled,
              onChanged: (_) => _onAddressFieldChanged(),
              validator: (value) {
                if (value != null && value.isNotEmpty && value.length < 5) {
                  return 'Please enter a valid postal code';
                }
                return null;
              },
            ),
          ],
        ],
      ),
    );
  }

  /// Validate the form
  bool validate() {
    return _formKey.currentState?.validate() ?? false;
  }

  /// Get the current address data
  NigeriaAddressData get currentAddress {
    return NigeriaAddressData(
      streetAddress:
          widget.showStreetAddress ? _streetController.text.trim() : null,
      state: _selectedState,
      lga: _selectedLGA,
      ward: _selectedWard,
      postalCode:
          widget.showPostalCode ? _postalCodeController.text.trim() : null,
    );
  }
}

/// Data class representing a Nigerian address
class NigeriaAddressData {
  final String? streetAddress;
  final NigerianState? state;
  final LGA? lga;
  final Ward? ward;
  final String? postalCode;

  const NigeriaAddressData({
    this.streetAddress,
    this.state,
    this.lga,
    this.ward,
    this.postalCode,
  });

  /// Check if the address is complete (has state, LGA, and ward)
  bool get isComplete => state != null && lga != null && ward != null;

  /// Check if the address is empty
  bool get isEmpty =>
      state == null &&
      lga == null &&
      ward == null &&
      (streetAddress == null || streetAddress!.isEmpty) &&
      (postalCode == null || postalCode!.isEmpty);

  /// Get formatted address string
  String get formattedAddress {
    final parts = <String>[];

    if (streetAddress != null && streetAddress!.isNotEmpty) {
      parts.add(streetAddress!);
    }

    if (ward != null) parts.add(ward!.name);
    if (lga != null) parts.add(lga!.name);
    if (state != null) parts.add(state!.name);

    if (postalCode != null && postalCode!.isNotEmpty) {
      parts.add(postalCode!);
    }

    return parts.join(', ');
  }

  @override
  String toString() => formattedAddress;

  /// Create a copy with some fields replaced
  NigeriaAddressData copyWith({
    String? streetAddress,
    NigerianState? state,
    LGA? lga,
    Ward? ward,
    String? postalCode,
  }) {
    return NigeriaAddressData(
      streetAddress: streetAddress ?? this.streetAddress,
      state: state ?? this.state,
      lga: lga ?? this.lga,
      ward: ward ?? this.ward,
      postalCode: postalCode ?? this.postalCode,
    );
  }
}
