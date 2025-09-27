import 'package:flutter/material.dart';
import '../client/nigeria_geo_client.dart';
import '../models/search_result.dart';
import '../utils/debouncer.dart';

/// A search widget for searching Nigerian geographic locations
class NigeriaGeoSearch extends StatefulWidget {
  /// The Nigeria Geo Client instance
  final NigeriaGeoClient client;

  /// Callback when a search result is selected
  final void Function(SearchResultItem) onResultSelected;

  /// Hint text for the search field
  final String hintText;

  /// Minimum characters to trigger search
  final int minSearchLength;

  /// Search debounce duration
  final Duration searchDebounce;

  /// Maximum number of results to show
  final int maxResults;

  const NigeriaGeoSearch({
    super.key,
    required this.client,
    required this.onResultSelected,
    this.hintText = 'Search states, LGAs, wards...',
    this.minSearchLength = 2,
    this.searchDebounce = const Duration(milliseconds: 500),
    this.maxResults = 10,
  });

  @override
  State<NigeriaGeoSearch> createState() => _NigeriaGeoSearchState();
}

class _NigeriaGeoSearchState extends State<NigeriaGeoSearch> {
  final TextEditingController _controller = TextEditingController();
  final FocusNode _focusNode = FocusNode();
  final OverlayPortalController _overlayController = OverlayPortalController();

  List<SearchResultItem> _results = [];
  bool _isLoading = false;
  String? _error;
  Debouncer? _debouncer;

  @override
  void initState() {
    super.initState();
    _debouncer = Debouncer(delay: widget.searchDebounce);
    _controller.addListener(_onSearchChanged);
    _focusNode.addListener(_onFocusChanged);
  }

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    _debouncer?.cancel();
    super.dispose();
  }

  void _onSearchChanged() {
    final query = _controller.text.trim();

    if (query.length < widget.minSearchLength) {
      setState(() {
        _results = [];
        _error = null;
      });
      _overlayController.hide();
      return;
    }

    _debouncer?.call(() => _performSearch(query));
  }

  void _onFocusChanged() {
    if (_focusNode.hasFocus && _results.isNotEmpty) {
      _overlayController.show();
    } else {
      _overlayController.hide();
    }
  }

  Future<void> _performSearch(String query) async {
    try {
      setState(() {
        _isLoading = true;
        _error = null;
      });

      final response = await widget.client.searchAll(query);
      final items = <SearchResultItem>[];

      // Convert states to SearchResultItems
      for (final state in response.states.take(widget.maxResults)) {
        items.add(SearchResultItem(type: SearchType.state, data: state));
      }

      // Convert LGAs to SearchResultItems
      for (final lga in response.lgas.take(widget.maxResults)) {
        items.add(SearchResultItem(type: SearchType.lga, data: lga));
      }

      // Convert wards to SearchResultItems
      for (final ward in response.wards.take(widget.maxResults)) {
        items.add(SearchResultItem(type: SearchType.ward, data: ward));
      }

      // Convert postal codes to SearchResultItems
      for (final postal in response.postalCodes.take(widget.maxResults)) {
        items.add(SearchResultItem(type: SearchType.postalCode, data: postal));
      }

      setState(() {
        _results = items.take(widget.maxResults).toList();
        _isLoading = false;
      });

      if (_focusNode.hasFocus) {
        _overlayController.show();
      }
    } catch (e) {
      setState(() {
        _error = e.toString();
        _isLoading = false;
        _results = [];
      });
    }
  }

  void _selectResult(SearchResultItem result) {
    _controller.text = result.title;
    _overlayController.hide();
    _focusNode.unfocus();
    widget.onResultSelected(result);
  }

  @override
  Widget build(BuildContext context) {
    return OverlayPortal(
      controller: _overlayController,
      overlayChildBuilder: (context) => _buildOverlay(context),
      child: TextFormField(
        controller: _controller,
        focusNode: _focusNode,
        decoration: InputDecoration(
          hintText: widget.hintText,
          prefixIcon: const Icon(Icons.search),
          suffixIcon: _isLoading
              ? const SizedBox(
                  width: 20,
                  height: 20,
                  child: Padding(
                    padding: EdgeInsets.all(12.0),
                    child: CircularProgressIndicator(strokeWidth: 2),
                  ),
                )
              : _controller.text.isNotEmpty
                  ? IconButton(
                      icon: const Icon(Icons.clear),
                      onPressed: () {
                        _controller.clear();
                        _overlayController.hide();
                      },
                    )
                  : null,
          border: const OutlineInputBorder(),
          errorText: _error,
        ),
      ),
    );
  }

  Widget _buildOverlay(BuildContext context) {
    final renderBox = context.findRenderObject() as RenderBox?;
    if (renderBox == null) return const SizedBox.shrink();

    final size = renderBox.size;
    final position = renderBox.localToGlobal(Offset.zero);

    return Positioned(
      left: position.dx,
      top: position.dy + size.height + 4,
      width: size.width,
      child: Material(
        elevation: 8,
        borderRadius: BorderRadius.circular(8),
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxHeight: 300),
          child: _results.isEmpty && !_isLoading
              ? const Padding(
                  padding: EdgeInsets.all(16.0),
                  child: Text(
                    'No results found',
                    style: TextStyle(color: Colors.grey),
                  ),
                )
              : ListView.builder(
                  shrinkWrap: true,
                  itemCount: _results.length,
                  itemBuilder: (context, index) {
                    final result = _results[index];
                    return ListTile(
                      leading: _getTypeIcon(result.type),
                      title: Text(result.title),
                      subtitle: Text(result.subtitle),
                      onTap: () => _selectResult(result),
                      dense: true,
                    );
                  },
                ),
        ),
      ),
    );
  }

  Icon _getTypeIcon(SearchType type) {
    switch (type) {
      case SearchType.state:
        return const Icon(Icons.location_city, color: Colors.blue);
      case SearchType.lga:
        return const Icon(Icons.location_on, color: Colors.green);
      case SearchType.ward:
        return const Icon(Icons.place, color: Colors.orange);
      case SearchType.postalCode:
        return const Icon(Icons.local_post_office, color: Colors.purple);
    }
  }
}
