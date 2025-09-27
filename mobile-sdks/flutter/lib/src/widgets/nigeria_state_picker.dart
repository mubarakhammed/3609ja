import 'package:flutter/material.dart';
import '../models/state.dart';
import '../client/nigeria_geo_client.dart';

/// A dropdown widget for selecting Nigerian states
class NigeriaStatePicker extends StatefulWidget {
  /// The currently selected state
  final NigerianState? selectedState;

  /// Callback when a state is selected
  final void Function(NigerianState) onStateSelected;

  /// The Nigeria Geo Client instance
  final NigeriaGeoClient client;

  /// Hint text for the dropdown
  final String hintText;

  /// Error text to display
  final String? errorText;

  /// Whether to show loading indicator
  final bool enabled;

  const NigeriaStatePicker({
    super.key,
    required this.onStateSelected,
    required this.client,
    this.selectedState,
    this.hintText = 'Select a state',
    this.errorText,
    this.enabled = true,
  });

  @override
  State<NigeriaStatePicker> createState() => _NigeriaStatePickerState();
}

class _NigeriaStatePickerState extends State<NigeriaStatePicker> {
  List<NigerianState>? _states;
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _loadStates();
  }

  Future<void> _loadStates() async {
    try {
      setState(() {
        _isLoading = true;
        _error = null;
      });

      final result = await widget.client.getStates();
      setState(() {
        _states = result.data;
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
        DropdownButtonFormField<NigerianState>(
          value: widget.selectedState,
          decoration: InputDecoration(
            hintText: widget.hintText,
            errorText: widget.errorText ?? _error,
            border: const OutlineInputBorder(),
          ),
          items: _isLoading
              ? null
              : _states?.map((state) {
                  return DropdownMenuItem<NigerianState>(
                    value: state,
                    child: Text(state.name),
                  );
                }).toList(),
          onChanged: widget.enabled && !_isLoading
              ? (NigerianState? state) {
                  if (state != null) {
                    widget.onStateSelected(state);
                  }
                }
              : null,
          validator: (value) {
            if (value == null) {
              return 'Please select a state';
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
                  onPressed: _loadStates,
                  child: const Text('Retry'),
                ),
              ],
            ),
          ),
      ],
    );
  }
}
