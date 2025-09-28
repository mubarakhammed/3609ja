import 'package:flutter/material.dart';
import '../models/lga.dart';
import '../models/state.dart';
import '../client/nigeria_geo_client.dart';

/// A dropdown widget for selecting Local Government Areas
class NigeriaLGAPicker extends StatefulWidget {
  /// The currently selected LGA
  final LGA? selectedLGA;

  /// Callback when an LGA is selected
  final void Function(LGA) onLGASelected;

  /// The Nigeria Geo Client instance
  final NigeriaGeoClient client;

  /// The selected state to filter LGAs
  final NigerianState? state;

  /// Hint text for the dropdown
  final String hintText;

  /// Error text to display
  final String? errorText;

  /// Whether to show loading indicator
  final bool enabled;

  const NigeriaLGAPicker({
    super.key,
    required this.onLGASelected,
    required this.client,
    this.selectedLGA,
    this.state,
    this.hintText = 'Select an LGA',
    this.errorText,
    this.enabled = true,
  });

  @override
  State<NigeriaLGAPicker> createState() => _NigeriaLGAPickerState();
}

class _NigeriaLGAPickerState extends State<NigeriaLGAPicker> {
  List<LGA>? _lgas;
  bool _isLoading = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    if (widget.state != null) {
      _loadLGAs();
    }
  }

  @override
  void didUpdateWidget(NigeriaLGAPicker oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.state != oldWidget.state) {
      if (widget.state != null) {
        _loadLGAs();
      } else {
        setState(() {
          _lgas = null;
          _error = null;
        });
      }
    }
  }

  Future<void> _loadLGAs() async {
    if (widget.state == null) return;

    try {
      setState(() {
        _isLoading = true;
        _error = null;
        _lgas = null;
      });

      final result = await widget.client
          .getLGAs(stateId: widget.state!.id, limit: 50); // Fetch all LGAs
      setState(() {
        _lgas = result.data;
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
        DropdownButtonFormField<LGA>(
          value: widget.selectedLGA,
          decoration: InputDecoration(
            hintText: widget.state == null
                ? 'Please select a state first'
                : widget.hintText,
            errorText: widget.errorText ?? _error,
            border: const OutlineInputBorder(),
          ),
          items: _isLoading || widget.state == null
              ? null
              : _lgas?.map((lga) {
                  return DropdownMenuItem<LGA>(
                    value: lga,
                    child: Text(lga.name),
                  );
                }).toList(),
          onChanged: widget.enabled && !_isLoading && widget.state != null
              ? (LGA? lga) {
                  if (lga != null) {
                    widget.onLGASelected(lga);
                  }
                }
              : null,
          validator: (value) {
            if (widget.state != null && value == null) {
              return 'Please select an LGA';
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
                  onPressed: _loadLGAs,
                  child: const Text('Retry'),
                ),
              ],
            ),
          ),
      ],
    );
  }
}
