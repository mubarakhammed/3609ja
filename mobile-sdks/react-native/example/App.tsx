import React, { useState, useEffect } from 'react';
import { ScrollView, StyleSheet, View } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import {
  Provider as PaperProvider,
  Appbar,
  Card,
  Text,
  Button,
  TextInput,
  SegmentedButtons,
  List,
  ActivityIndicator
} from 'react-native-paper';
import { Picker } from '@react-native-picker/picker';

// Import the SDK - zero config!
import { nigeriaGeoSDK, State, Lga, Ward, PostalCode, SearchResult } from '../sdk';

export default function App() {
  const [selectedTab, setSelectedTab] = useState('api');

  return (
    <PaperProvider>
      <Appbar.Header style={styles.header}>
        <Appbar.Content
          title="🇳🇬 Nigeria Geo SDK"
          titleStyle={styles.headerTitle}
        />
      </Appbar.Header>

      <View style={styles.container}>
        <SegmentedButtons
          value={selectedTab}
          onValueChange={setSelectedTab}
          buttons={[
            { value: 'api', label: 'API Demo' },
            { value: 'widgets', label: 'Widgets' },
            { value: 'address', label: 'Address' },
            { value: 'search', label: 'Search' },
          ]}
          style={styles.tabs}
        />

        <ScrollView style={styles.content}>
          {selectedTab === 'api' && <ApiDemoTab />}
          {selectedTab === 'widgets' && <WidgetsTab />}
          {selectedTab === 'address' && <AddressTab />}
          {selectedTab === 'search' && <SearchTab />}
        </ScrollView>
      </View>

      <StatusBar style="auto" />
    </PaperProvider>
  );
}

function ApiDemoTab() {
  const [states, setStates] = useState<State[]>([]);
  const [loading, setLoading] = useState(false);

  const loadStates = async () => {
    setLoading(true);
    try {
      const data = await nigeriaGeoSDK.getStates();
      setStates(data);
    } catch (error) {
      console.error('Error loading states:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadStates();
  }, []);

  return (
    <View>
      <Card style={styles.card}>
        <Card.Title title="States API" />
        <Card.Content>
          <Button onPress={loadStates} mode="contained" disabled={loading}>
            {loading ? 'Loading...' : 'Reload States'}
          </Button>

          {loading && <ActivityIndicator style={styles.loader} />}

          <Text style={styles.resultsTitle}>
            Results ({states.length} states):
          </Text>

          {states.slice(0, 5).map((state) => (
            <List.Item
              key={state.id}
              title={state.name}
              description={`Code: ${state.code}`}
              left={(props) => <List.Icon {...props} icon="map" />}
            />
          ))}

          {states.length > 5 && (
            <Text style={styles.moreText}>
              ... and {states.length - 5} more states
            </Text>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

function WidgetsTab() {
  const [selectedState, setSelectedState] = useState<State | null>(null);
  const [selectedLga, setSelectedLga] = useState<Lga | null>(null);
  const [selectedWard, setSelectedWard] = useState<Ward | null>(null);

  return (
    <View>
      <Card style={styles.card}>
        <Card.Title title="Hierarchical Geographic Dropdowns" />
        <Card.Content>
          <Text style={styles.stepTitle}>1. Select State:</Text>
          <StateDropdown
            onStateChange={(state) => {
              setSelectedState(state);
              setSelectedLga(null);
              setSelectedWard(null);
            }}
            selectedState={selectedState}
          />

          <Text style={styles.stepTitle}>2. Select LGA:</Text>
          <LgaDropdown
            stateId={selectedState?.id || ''}
            onLgaChange={(lga) => {
              setSelectedLga(lga);
              setSelectedWard(null);
            }}
            selectedLga={selectedLga}
          />

          <Text style={styles.stepTitle}>3. Select Ward:</Text>
          <WardDropdown
            lgaId={selectedLga?.id}
            onWardChange={setSelectedWard}
            selectedWard={selectedWard}
          />

          {(selectedState || selectedLga || selectedWard) && (
            <Card style={[styles.card, styles.selectionCard]}>
              <Card.Content>
                <Text style={styles.selectionTitle}>Selection Summary:</Text>
                {selectedState && (
                  <Text style={styles.selectionText}>
                    State: {selectedState.name} ({selectedState.code})
                  </Text>
                )}
                {selectedLga && (
                  <Text style={styles.selectionText}>
                    LGA: {selectedLga.name}
                  </Text>
                )}
                {selectedWard && (
                  <Text style={styles.selectionText}>
                    Ward: {selectedWard.name}
                  </Text>
                )}
              </Card.Content>
            </Card>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

function AddressTab() {
  const [selectedState, setSelectedState] = useState<State | null>(null);
  const [selectedLga, setSelectedLga] = useState<Lga | null>(null);
  const [selectedWard, setSelectedWard] = useState<Ward | null>(null);
  const [streetAddress, setStreetAddress] = useState('');
  const [city, setCity] = useState('');

  const resetForm = () => {
    setSelectedState(null);
    setSelectedLga(null);
    setSelectedWard(null);
    setStreetAddress('');
    setCity('');
  };

  const isFormComplete = selectedState && selectedLga && selectedWard && streetAddress.trim() && city.trim();

  return (
    <View>
      <Card style={styles.card}>
        <Card.Title title="Complete Address Form" />
        <Card.Content>
          <Text style={styles.stepTitle}>Geographic Information:</Text>

          <StateDropdown
            onStateChange={(state) => {
              setSelectedState(state);
              setSelectedLga(null);
              setSelectedWard(null);
            }}
            selectedState={selectedState}

          />

          <LgaDropdown
            stateId={selectedState?.id || ''}
            onLgaChange={(lga) => {
              setSelectedLga(lga);
              setSelectedWard(null);
            }}
            selectedLga={selectedLga}

          />

          <WardDropdown
            lgaId={selectedLga?.id}
            onWardChange={setSelectedWard}
            selectedWard={selectedWard}

          />

          <Text style={styles.stepTitle}>Address Details:</Text>

          <TextInput
            label="Street Address"
            value={streetAddress}
            onChangeText={setStreetAddress}
            placeholder="Enter your street address"
            style={styles.addressInput}
          />

          <TextInput
            label="City/Town"
            value={city}
            onChangeText={setCity}
            placeholder="Enter your city or town"
            style={styles.addressInput}
          />

          <View style={styles.buttonRow}>
            <Button
              mode="outlined"
              onPress={resetForm}
              style={styles.button}
            >
              Reset Form
            </Button>

            <Button
              mode="contained"
              onPress={() => {
                console.log('Address submitted:', {
                  state: selectedState,
                  lga: selectedLga,
                  ward: selectedWard,
                  streetAddress,
                  city
                });
              }}
              disabled={!isFormComplete}
              style={styles.button}
            >
              Submit Address
            </Button>
          </View>

          {isFormComplete && (
            <Card style={[styles.card, styles.addressPreview]}>
              <Card.Content>
                <Text style={styles.previewTitle}>Address Preview:</Text>
                <Text style={styles.previewText}>
                  {streetAddress}
                </Text>
                <Text style={styles.previewText}>
                  {city}, {selectedWard?.name} Ward
                </Text>
                <Text style={styles.previewText}>
                  {selectedLga?.name} LGA, {selectedState?.name} State
                </Text>
                <Text style={styles.previewText}>
                  Nigeria
                </Text>
              </Card.Content>
            </Card>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

function SearchTab() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;

    setLoading(true);
    try {
      const data = await nigeriaGeoSDK.search(query);
      setResults(data);
    } catch (error) {
      console.error('Search error:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <View>
      <Card style={styles.card}>
        <Card.Title title="Search" />
        <Card.Content>
          <TextInput
            label="Search locations..."
            value={query}
            onChangeText={setQuery}
            right={
              <TextInput.Icon
                icon="magnify"
                onPress={handleSearch}
                disabled={loading}
              />
            }
            style={styles.searchInput}
          />

          {loading && <ActivityIndicator style={styles.loader} />}

          {results && (
            <View style={styles.searchResults}>
              <Text variant="titleMedium">Results:</Text>
              <Text>States: {results.states?.length || 0}</Text>
              <Text>LGAs: {results.lgas?.length || 0}</Text>
              <Text>Wards: {results.wards?.length || 0}</Text>
              <Text>Postal Codes: {results.postal_codes?.length || 0}</Text>
            </View>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

// Simple dropdown components
function StateDropdown({ onStateChange, selectedState }: {
  onStateChange: (state: State | null) => void;
  selectedState: State | null;
}) {
  const [states, setStates] = useState<State[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadStates = async () => {
      try {
        const data = await nigeriaGeoSDK.getStates();
        setStates(data);
      } catch (error) {
        console.error('Error loading states:', error);
      } finally {
        setLoading(false);
      }
    };

    loadStates();
  }, []);

  if (loading) {
    return <ActivityIndicator style={styles.loader} />;
  }

  return (
    <View style={styles.dropdown}>
      <Text variant="labelMedium">Select State:</Text>
      <Picker
        selectedValue={selectedState?.id || ''}
        onValueChange={(itemValue) => {
          if (itemValue) {
            const state = states.find(s => s.id === itemValue);
            onStateChange(state || null);
          } else {
            onStateChange(null);
          }
        }}
        style={styles.picker}
      >
        <Picker.Item label="Select a state..." value="" />
        {states.map((state) => (
          <Picker.Item key={state.id} label={state.name} value={state.id} />
        ))}
      </Picker>
    </View>
  );
}

function LgaDropdown({ stateId, onLgaChange, selectedLga }: {
  stateId: string;
  onLgaChange: (lga: Lga | null) => void;
  selectedLga: Lga | null;
}) {
  const [lgas, setLgas] = useState<Lga[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!stateId) {
      setLgas([]);
      return;
    }

    const loadLgas = async () => {
      setLoading(true);
      try {
        const data = await nigeriaGeoSDK.getLgasByState(stateId);
        setLgas(data);
      } catch (error) {
        console.error('Error loading LGAs:', error);
      } finally {
        setLoading(false);
      }
    };

    loadLgas();
  }, [stateId]);

  if (loading) {
    return <ActivityIndicator style={styles.loader} />;
  }

  return (
    <View style={styles.dropdown}>
      <Text variant="labelMedium">Select LGA:</Text>
      <Picker
        selectedValue={selectedLga?.id || ''}
        onValueChange={(itemValue) => {
          if (itemValue) {
            const lga = lgas.find(l => l.id === itemValue);
            onLgaChange(lga || null);
          } else {
            onLgaChange(null);
          }
        }}
        style={styles.picker}
        enabled={!!stateId}
      >
        <Picker.Item label="Select an LGA..." value="" />
        {lgas.map((lga) => (
          <Picker.Item key={lga.id} label={lga.name} value={lga.id} />
        ))}
      </Picker>
    </View>
  );
}

function WardDropdown({ lgaId, onWardChange, selectedWard }: {
  lgaId?: string;
  onWardChange: (ward: Ward | null) => void;
  selectedWard: Ward | null;
}) {
  const [wards, setWards] = useState<Ward[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!lgaId) {
      setWards([]);
      return;
    }

    const loadWards = async () => {
      setLoading(true);
      try {
        const data = await nigeriaGeoSDK.getWardsByLga(lgaId);
        setWards(data);
      } catch (error) {
        console.error('Error loading wards:', error);
      } finally {
        setLoading(false);
      }
    };

    loadWards();
  }, [lgaId]);

  if (loading) {
    return <ActivityIndicator style={styles.loader} />;
  }

  return (
    <View style={styles.dropdown}>
      <Text variant="labelMedium">Select Ward:</Text>
      <Picker
        selectedValue={selectedWard?.id || ''}
        onValueChange={(itemValue) => {
          if (itemValue) {
            const ward = wards.find(w => w.id === itemValue);
            onWardChange(ward || null);
          } else {
            onWardChange(null);
          }
        }}
        style={styles.picker}
        enabled={!!lgaId}
      >
        <Picker.Item label="Select a ward..." value="" />
        {wards.map((ward) => (
          <Picker.Item key={ward.id} label={ward.name} value={ward.id} />
        ))}
      </Picker>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  header: {
    backgroundColor: '#2E7D32',
  },
  headerTitle: {
    color: 'white',
    fontWeight: 'bold',
  },
  tabs: {
    margin: 16,
  },
  content: {
    flex: 1,
    padding: 16,
  },
  card: {
    marginBottom: 16,
  },
  loader: {
    marginVertical: 16,
  },
  resultsTitle: {
    marginTop: 16,
    marginBottom: 8,
    fontWeight: 'bold',
  },
  moreText: {
    fontStyle: 'italic',
    color: '#666',
    marginTop: 8,
  },
  selection: {
    marginTop: 16,
    padding: 12,
    backgroundColor: '#e8f5e8',
    borderRadius: 8,
    fontWeight: 'bold',
  },
  searchInput: {
    marginBottom: 16,
  },
  searchResults: {
    marginTop: 16,
    padding: 12,
    backgroundColor: '#f0f0f0',
    borderRadius: 8,
  },
  dropdown: {
    marginVertical: 8,
  },
  picker: {
    backgroundColor: '#f5f5f5',
    borderRadius: 8,
    marginTop: 4,
  },
  stepTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    marginTop: 16,
    marginBottom: 8,
    color: '#2E7D32',
  },
  selectionCard: {
    backgroundColor: '#e8f5e8',
    marginTop: 16,
  },
  selectionTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 8,
    color: '#2E7D32',
  },
  selectionText: {
    fontSize: 14,
    marginBottom: 4,
    color: '#1B5E20',
  },
  addressInput: {
    marginVertical: 8,
  },
  buttonRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginTop: 16,
  },
  button: {
    flex: 1,
    marginHorizontal: 4,
  },
  addressPreview: {
    backgroundColor: '#e3f2fd',
    marginTop: 16,
  },
  previewTitle: {
    fontSize: 16,
    fontWeight: 'bold',
    marginBottom: 8,
    color: '#1565C0',
  },
  previewText: {
    fontSize: 14,
    marginBottom: 2,
    color: '#0D47A1',
  },
});
