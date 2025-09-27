import 'package:flutter/material.dart';
import 'package:nigeria_geo_sdk/nigeria_geo_sdk.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize the SDK - zero configuration required!
  await NigeriaGeoSDK.initialize();

  runApp(NigeriaGeoApp());
}

class NigeriaGeoApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Nigeria Geo SDK Demo',
      theme: ThemeData(
        primarySwatch: Colors.green,
        useMaterial3: true,
      ),
      home: HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  final NigeriaGeoClient client = NigeriaGeoSDK.client;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(
          children: [
            Text('🇳🇬'),
            SizedBox(width: 8),
            Text('Nigeria Geo SDK'),
          ],
        ),
        backgroundColor: Colors.green,
        foregroundColor: Colors.white,
        bottom: TabBar(
          controller: _tabController,
          labelColor: Colors.white,
          unselectedLabelColor: Colors.white70,
          indicatorColor: Colors.white,
          tabs: [
            Tab(icon: Icon(Icons.api), text: 'API'),
            Tab(icon: Icon(Icons.widgets), text: 'Widgets'),
            Tab(icon: Icon(Icons.location_on), text: 'Address'),
            Tab(icon: Icon(Icons.search), text: 'Search'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          APIDemo(),
          WidgetsDemo(),
          AddressDemo(),
          SearchDemo(),
        ],
      ),
    );
  }
}

// API Demo Tab
class APIDemo extends StatefulWidget {
  @override
  _APIDemoState createState() => _APIDemoState();
}

class _APIDemoState extends State<APIDemo> {
  final NigeriaGeoClient client = NigeriaGeoSDK.client;
  String _output = 'Tap a button to test the API...';
  bool _isLoading = false;

  Future<void> _testStatesAPI() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final response = await client.getStates(limit: 5);
      setState(() {
        _output = '''✅ States API Success!
Found ${response.data.length} states (page ${response.pagination.page}):

${response.data.map((s) => '• ${s.name} (${s.code})').join('\n')}

Total states: ${response.pagination.total}''';
      });
    } catch (e) {
      setState(() {
        _output = '❌ Error: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  Future<void> _testSearchAPI() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final result = await client.searchAll('Lagos');
      setState(() {
        _output = '''✅ Search API Success!
Query: "Lagos"

States found: ${result.states.length}
LGAs found: ${result.lgas.length}
Wards found: ${result.wards.length}
Postal codes found: ${result.postalCodes.length}

Sample results:
${result.states.take(2).map((s) => '• State: ${s.name}').join('\n')}
${result.lgas.take(2).map((l) => '• LGA: ${l.name}').join('\n')}''';
      });
    } catch (e) {
      setState(() {
        _output = '❌ Search Error: $e';
      });
    } finally {
      setState(() {
        _isLoading = false;
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.all(16),
      child: Column(
        children: [
          Row(
            children: [
              Expanded(
                child: ElevatedButton(
                  onPressed: _isLoading ? null : _testStatesAPI,
                  child: Text('Test States API'),
                ),
              ),
              SizedBox(width: 12),
              Expanded(
                child: ElevatedButton(
                  onPressed: _isLoading ? null : _testSearchAPI,
                  child: Text('Test Search API'),
                ),
              ),
            ],
          ),
          SizedBox(height: 16),
          Expanded(
            child: Container(
              width: double.infinity,
              padding: EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.grey[100],
                borderRadius: BorderRadius.circular(8),
                border: Border.all(color: Colors.grey[300]!),
              ),
              child: _isLoading
                  ? Center(child: CircularProgressIndicator())
                  : SingleChildScrollView(
                      child: Text(
                        _output,
                        style: TextStyle(fontFamily: 'monospace', fontSize: 14),
                      ),
                    ),
            ),
          ),
        ],
      ),
    );
  }
}

// Widgets Demo Tab
class WidgetsDemo extends StatefulWidget {
  @override
  _WidgetsDemoState createState() => _WidgetsDemoState();
}

class _WidgetsDemoState extends State<WidgetsDemo> {
  final NigeriaGeoClient client = NigeriaGeoSDK.client;
  NigerianState? selectedState;
  LGA? selectedLGA;
  Ward? selectedWard;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Hierarchical Selection Widgets',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 16),
          Text('1. Select State:',
              style: TextStyle(fontWeight: FontWeight.w500)),
          SizedBox(height: 8),
          NigeriaStatePicker(
            client: client,
            selectedState: selectedState,
            onStateSelected: (state) {
              setState(() {
                selectedState = state;
                selectedLGA = null;
                selectedWard = null;
              });
            },
          ),
          SizedBox(height: 16),
          Text('2. Select LGA:', style: TextStyle(fontWeight: FontWeight.w500)),
          SizedBox(height: 8),
          NigeriaLGAPicker(
            client: client,
            state: selectedState,
            selectedLGA: selectedLGA,
            onLGASelected: (lga) {
              setState(() {
                selectedLGA = lga;
                selectedWard = null;
              });
            },
          ),
          SizedBox(height: 16),
          Text('3. Select Ward:',
              style: TextStyle(fontWeight: FontWeight.w500)),
          SizedBox(height: 8),
          NigeriaWardPicker(
            client: client,
            lga: selectedLGA,
            selectedWard: selectedWard,
            onWardSelected: (ward) {
              setState(() {
                selectedWard = ward;
              });
            },
          ),
          SizedBox(height: 24),
          if (selectedState != null ||
              selectedLGA != null ||
              selectedWard != null) ...[
            Card(
              color: Colors.blue[50],
              child: Padding(
                padding: EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text('Selection Summary:',
                        style: TextStyle(fontWeight: FontWeight.bold)),
                    SizedBox(height: 8),
                    if (selectedState != null)
                      Text(
                          'State: ${selectedState!.name} (${selectedState!.code})'),
                    if (selectedLGA != null) Text('LGA: ${selectedLGA!.name}'),
                    if (selectedWard != null)
                      Text('Ward: ${selectedWard!.name}'),
                  ],
                ),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

// Address Demo Tab
class AddressDemo extends StatefulWidget {
  @override
  _AddressDemoState createState() => _AddressDemoState();
}

class _AddressDemoState extends State<AddressDemo> {
  final NigeriaGeoClient client = NigeriaGeoSDK.client;
  NigeriaAddressData? currentAddress;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.all(16),
      child: Column(
        children: [
          Text(
            'Complete Address Form',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 16),
          Expanded(
            child: NigeriaAddressForm(
              client: client,
              showStreetAddress: true,
              showPostalCode: true,
              onAddressChanged: (address) {
                setState(() {
                  currentAddress = address;
                });
              },
            ),
          ),
          if (currentAddress != null && !currentAddress!.isEmpty) ...[
            Divider(),
            Card(
              color: Colors.green[50],
              child: Padding(
                padding: EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Icon(Icons.location_on, color: Colors.green),
                        SizedBox(width: 8),
                        Text('Address Preview:',
                            style: TextStyle(fontWeight: FontWeight.bold)),
                      ],
                    ),
                    SizedBox(height: 8),
                    Text(currentAddress!.formattedAddress),
                    SizedBox(height: 8),
                    Row(
                      children: [
                        Icon(
                          currentAddress!.isComplete
                              ? Icons.check_circle
                              : Icons.warning,
                          color: currentAddress!.isComplete
                              ? Colors.green
                              : Colors.orange,
                          size: 16,
                        ),
                        SizedBox(width: 4),
                        Text(
                          currentAddress!.isComplete
                              ? 'Complete ✅'
                              : 'Incomplete ⚠️',
                          style: TextStyle(
                            color: currentAddress!.isComplete
                                ? Colors.green
                                : Colors.orange,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ),
          ],
          SizedBox(height: 16),
          ElevatedButton(
            onPressed: currentAddress?.isComplete == true
                ? () {
                    ScaffoldMessenger.of(context).showSnackBar(
                      SnackBar(
                        content: Text(
                            'Address saved! ${currentAddress!.formattedAddress}'),
                        backgroundColor: Colors.green,
                      ),
                    );
                  }
                : null,
            child: Text('Save Address'),
            style: ElevatedButton.styleFrom(
              minimumSize: Size(double.infinity, 48),
            ),
          ),
        ],
      ),
    );
  }
}

// Search Demo Tab
class SearchDemo extends StatelessWidget {
  final NigeriaGeoClient client = NigeriaGeoSDK.client;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            'Live Search Widget',
            style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
          ),
          SizedBox(height: 8),
          Text(
            'Search for states, LGAs, wards, or postal codes. Results appear as you type!',
            style: TextStyle(color: Colors.grey[600]),
          ),
          SizedBox(height: 16),
          NigeriaGeoSearch(
            client: client,
            hintText: 'Try searching "Lagos", "Ikeja", "Victoria Island"...',
            onResultSelected: (result) {
              showDialog(
                context: context,
                builder: (context) => AlertDialog(
                  title: Row(
                    children: [
                      Icon(_getTypeIcon(result.type),
                          color: _getTypeColor(result.type)),
                      SizedBox(width: 8),
                      Expanded(child: Text(result.title)),
                    ],
                  ),
                  content: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(result.subtitle),
                      SizedBox(height: 8),
                      Text(
                        'Type: ${result.type.toString().split('.').last}',
                        style: TextStyle(
                          fontWeight: FontWeight.w500,
                          color: _getTypeColor(result.type),
                        ),
                      ),
                    ],
                  ),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context),
                      child: Text('Close'),
                    ),
                  ],
                ),
              );
            },
          ),
          SizedBox(height: 24),
          Card(
            color: Colors.blue[50],
            child: Padding(
              padding: EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text('Search Features:',
                      style: TextStyle(fontWeight: FontWeight.bold)),
                  SizedBox(height: 8),
                  Text('• Real-time search as you type'),
                  Text('• Debounced API calls (500ms delay)'),
                  Text('• Search across all geographic entities'),
                  Text('• Visual icons for different result types'),
                  Text('• Overlay dropdown interface'),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  IconData _getTypeIcon(SearchType type) {
    switch (type) {
      case SearchType.state:
        return Icons.location_city;
      case SearchType.lga:
        return Icons.location_on;
      case SearchType.ward:
        return Icons.place;
      case SearchType.postalCode:
        return Icons.local_post_office;
    }
  }

  Color _getTypeColor(SearchType type) {
    switch (type) {
      case SearchType.state:
        return Colors.blue;
      case SearchType.lga:
        return Colors.green;
      case SearchType.ward:
        return Colors.orange;
      case SearchType.postalCode:
        return Colors.purple;
    }
  }
}
