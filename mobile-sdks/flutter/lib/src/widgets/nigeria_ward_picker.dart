import 'package:flutter/material.dart';
import '../models/ward.dart';
import '../models/lga.dart';
import '../client/nigeria_geo_client.dart';

/// A dropdown widget for selecting Wards
class NigeriaWardPicker extends StatefulWidget {
  /// The currently selected ward
  final Ward? selectedWard;

  /// Callback when a ward is selected
  final void Function(Ward) onWardSelected;

  /// The Nigeria Geo Client instance
  final NigeriaGeoClient client;

  /// The selected LGA to filter wards
  final LGA? lga;

  /// Hint text for the dropdown
  final String hintText;

  /// Error text to display
  final String? errorText;

  /// Whether to show loading indicator
  final bool enabled;

  const NigeriaWardPicker({
    super.key,
    required this.onWardSelected,
    required this.client,
    this.selectedWard,
    this.lga,
    this.hintText = 'Select a ward',
    this.errorText,
    this.enabled = true,
  });

  @override
  State<NigeriaWardPicker> createState() => _NigeriaWardPickerState();
}

class _NigeriaWardPickerState extends State<NigeriaWardPicker> {
  List<Ward>? _wards;
  bool _isLoading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    if (widget.lga != null) {
      _loadWards();
    }
  }

  @override
  void didUpdateWidget(NigeriaWardPicker oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.lga != oldWidget.lga) {
      if (widget.lga != null) {
        _loadWards();
      } else {
        setState(() {
          _wards = null;
          _error = null;
        });
      }
    }
  }

  Future<void> _loadWards() async {
    if (widget.lga == null) return;

    try {
      setState(() {
        _isLoading = true;
        _error = null;
        _wards = null;
      });

      final result = await widget.client.getWards(lgaId: widget.lga!.id);
      setState(() {
        _wards = result.data;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        DropdownButtonFormField<Ward>(
          value: widget.selectedWard,
          decoration: InputDecoration(
            hintText: widget.lga == null
                ? 'Please select an LGA first'
                : widget.hintText,
            errorText: widget.errorText ?? _error,
            border: const OutlineInputBorder(),
          ),
          items: _isLoading || widget.lga == null
              ? null
              : _wards?.map((ward) {
                  return DropdownMenuItem<Ward>(
                    value: ward,
                    child: Text(ward.name),
                  );
                }).toList(),
          onChanged: widget.enabled && !_isLoading && widget.lga != null
              ? (Ward? ward) {
                  if (ward != null) {
                    widget.onWardSelected(ward);
                  }
                }
              : null,
          validator: (value) {
            if (widget.lga != null && value == null) {
              return 'Please select a ward';
            }
            return null;
          },
        ),
        if (_isLoading)
          const Padding(
            padding: EdgeInsets.only(top: 8.0),
            child: LinearProgressIndicator(),
          ),
        if (_error != null)
          Padding(
            padding: const EdgeInsets.only(top: 8.0),
            child: Row(
              children: [
                const Icon(Icons.error, color: Colors.red, size: 16),
                const SizedBox(width: 4),
                Expanded(
                  child: Text(
                    _error!,
                    style: const TextStyle(color: Colors.red, fontSize: 12),
                  ),
                ),
                TextButton(
                  onPressed: _loadWards,
                  child: const Text('Retry'),
                ),
              ],
            ),
          ),
      ],
    );
  }
}
