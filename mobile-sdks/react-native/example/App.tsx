/**
 * Nigeria Geo SDK Demo App - React Native
 * Exact same design as Flutter version with 4 tabs
 */

import React, { useState, useEffect } from 'react';
import { ScrollView, StyleSheet, View, Alert, Text as RNText } from 'react-native';
import { StatusBar } from 'expo-status-bar';
import {
  Provider as PaperProvider,
  Appbar,
  Card,
  Text,
  Button,
  TextInput,
  Divider,
  Chip,
  Surface,
  ActivityIndicator,
  SegmentedButtons
} from 'react-native-paper';

// Import SDK from package
import { NigeriaGeoSDK } from 'nigeria-geo-sdk';
import {
  StateDropdown,
  LgaDropdown,
  WardDropdown,
  PostalCodeDropdown
} from 'nigeria-geo-sdk';
import { State, Lga, Ward, PostalCode, SearchResult } from 'nigeria-geo-sdk'; export default function App() {
  const [selectedTab, setSelectedTab] = useState('api');

  return (
    <PaperProvider>
      <Appbar.Header style={styles.header}>
        <Appbar.Content
          title="🇳🇬 Nigeria Geo SDK"
          titleStyle={styles.headerTitle}
        />
      </Appbar.Header>

      <Surface style={styles.tabContainer}>
        <SegmentedButtons
          value={selectedTab}
          onValueChange={setSelectedTab}
          buttons={[
            { value: 'api', label: 'API', icon: 'api' },
            { value: 'widgets', label: 'Widgets', icon: 'widgets' },
            { value: 'address', label: 'Address', icon: 'map-marker' },
            { value: 'search', label: 'Search', icon: 'magnify' },
          ]}
          style={styles.segmentedButtons}
        />
      </Surface>

      <ScrollView style={styles.container}>
        {selectedTab === 'api' && <APIDemo />}
        {selectedTab === 'widgets' && <WidgetsDemo />}
        {selectedTab === 'address' && <AddressDemo />}
        {selectedTab === 'search' && <SearchDemo />}
      </ScrollView>

      <StatusBar style="light" />
    </PaperProvider>
  );
}

// API Demo Tab - Test raw API calls
function APIDemo() {
  const [output, setOutput] = useState('Tap a button to test the API...');
  const [isLoading, setIsLoading] = useState(false);
  const sdk = NigeriaGeoSDK.getInstance();

  const testAPI = async (apiCall: () => Promise<any>, description: string) => {
    setIsLoading(true);
    setOutput(`Testing ${description}...`);
    try {
      const result = await apiCall();
      setOutput(`✅ ${description}\n\n${JSON.stringify(result, null, 2)}`);
    } catch (error) {
      setOutput(`❌ ${description} failed:\n${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <View style={styles.tabContent}>
      <Card style={styles.card}>
        <Card.Title title="API Testing" subtitle="Test direct SDK API calls" />
        <Card.Content>
          <View style={styles.buttonRow}>
            <Button
              mode="contained"
              onPress={() => testAPI(() => sdk.getStates(), 'Get All States')}
              style={styles.apiButton}
              disabled={isLoading}
            >
              States
            </Button>
            <Button
              mode="contained"
              onPress={() => testAPI(() => sdk.getLgas(25), 'Get Lagos LGAs')}
              style={styles.apiButton}
              disabled={isLoading}
            >
              LGAs
            </Button>
          </View>
          <View style={styles.buttonRow}>
            <Button
              mode="contained"
              onPress={() => testAPI(() => sdk.getWards(317), 'Get Ikeja Wards')}
              style={styles.apiButton}
              disabled={isLoading}
            >
              Wards
            </Button>
            <Button
              mode="contained"
              onPress={() => testAPI(() => sdk.getPostalCodes(25), 'Get Lagos Postal Codes')}
              style={styles.apiButton}
              disabled={isLoading}
            >
              Postal
            </Button>
          </View>
          <Button
            mode="outlined"
            onPress={() => testAPI(() => sdk.search('lagos'), 'Search "lagos"')}
            style={styles.fullButton}
            disabled={isLoading}
          >
            Search Test
          </Button>
        </Card.Content>
      </Card>

      <Card style={styles.card}>
        <Card.Title title="API Response" />
        <Card.Content>
          {isLoading ? (
            <View style={styles.loadingContainer}>
              <ActivityIndicator size="large" />
              <Text style={styles.loadingText}>Testing API...</Text>
            </View>
          ) : (
            <Surface style={styles.outputContainer}>
              <RNText style={styles.outputText}>{output}</RNText>
            </Surface>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

// Widgets Demo Tab - Test UI components
function WidgetsDemo() {
  const [selectedState, setSelectedState] = useState<State | null>(null);
  const [selectedLga, setSelectedLga] = useState<Lga | null>(null);
  const [selectedWard, setSelectedWard] = useState<Ward | null>(null);
  const [selectedPostalCode, setSelectedPostalCode] = useState<PostalCode | null>(null);

  return (
    <View style={styles.tabContent}>
      <Card style={styles.card}>
        <Card.Title title="UI Widgets Demo" subtitle="Test pre-built components" />
        <Card.Content>
          <StateDropdown
            onStateChange={(state: State | null) => {
              setSelectedState(state);
              setSelectedLga(null);
              setSelectedWard(null);
            }}
            selectedState={selectedState}
          />

          <LgaDropdown
            stateId={selectedState?.id}
            onLgaChange={(lga: Lga | null) => {
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

          <PostalCodeDropdown
            stateId={selectedState?.id}
            onPostalCodeChange={setSelectedPostalCode}
            selectedPostalCode={selectedPostalCode}
          />
        </Card.Content>
      </Card>

      {(selectedState || selectedLga || selectedWard || selectedPostalCode) && (
        <Card style={styles.card}>
          <Card.Title title="Selected Values" />
          <Card.Content>
            {selectedState && (
              <Chip style={styles.chip} icon="map-marker">
                State: {selectedState.name} ({selectedState.code})
              </Chip>
            )}
            {selectedLga && (
              <Chip style={styles.chip} icon="city">
                LGA: {selectedLga.name}
              </Chip>
            )}
            {selectedWard && (
              <Chip style={styles.chip} icon="home-city">
                Ward: {selectedWard.name}
              </Chip>
            )}
            {selectedPostalCode && (
              <Chip style={styles.chip} icon="mailbox">
                Postal: {selectedPostalCode.code} - {selectedPostalCode.area}
              </Chip>
            )}
          </Card.Content>
        </Card>
      )}
    </View>
  );
}

// Address Demo Tab - Address validation
function AddressDemo() {
  const [selectedState, setSelectedState] = useState<State | null>(null);
  const [selectedLga, setSelectedLga] = useState<Lga | null>(null);
  const [selectedWard, setSelectedWard] = useState<Ward | null>(null);
  const [selectedPostalCode, setSelectedPostalCode] = useState<PostalCode | null>(null);
  const [validationResult, setValidationResult] = useState<boolean | null>(null);
  const [validationLoading, setValidationLoading] = useState(false);

  const sdk = NigeriaGeoSDK.getInstance();

  const validateAddress = async () => {
    if (!selectedState && !selectedLga && !selectedWard && !selectedPostalCode) {
      Alert.alert('Validation Error', 'Please select at least one geographical component');
      return;
    }

    setValidationLoading(true);
    try {
      const isValid = await sdk.validateAddress(
        selectedState?.name,
        selectedLga?.name,
        selectedWard?.name,
        selectedPostalCode?.code
      );
      setValidationResult(isValid);
    } catch (error) {
      Alert.alert('Validation Error', error instanceof Error ? error.message : 'Validation failed');
    } finally {
      setValidationLoading(false);
    }
  };

  return (
    <View style={styles.tabContent}>
      <Card style={styles.card}>
        <Card.Title title="Address Validation" subtitle="Build and validate addresses" />
        <Card.Content>
          <StateDropdown
            onStateChange={(state: State | null) => {
              setSelectedState(state);
              setSelectedLga(null);
              setSelectedWard(null);
              setValidationResult(null);
            }}
            selectedState={selectedState}
          />

          <LgaDropdown
            stateId={selectedState?.id}
            onLgaChange={(lga: Lga | null) => {
              setSelectedLga(lga);
              setSelectedWard(null);
              setValidationResult(null);
            }}
            selectedLga={selectedLga}
          />

          <WardDropdown
            lgaId={selectedLga?.id}
            onWardChange={(ward: Ward | null) => {
              setSelectedWard(ward);
              setValidationResult(null);
            }}
            selectedWard={selectedWard}
          />

          <PostalCodeDropdown
            stateId={selectedState?.id}
            onPostalCodeChange={(postalCode: PostalCode | null) => {
              setSelectedPostalCode(postalCode);
              setValidationResult(null);
            }}
            selectedPostalCode={selectedPostalCode}
          />

          <Button
            mode="contained"
            onPress={validateAddress}
            loading={validationLoading}
            disabled={validationLoading || (!selectedState && !selectedLga && !selectedWard && !selectedPostalCode)}
            style={styles.fullButton}
          >
            Validate Address
          </Button>

          {validationResult !== null && (
            <Surface style={[
              styles.validationResult,
              validationResult ? styles.validResult : styles.invalidResult
            ]}>
              <Text style={styles.validationText}>
                {validationResult ? '✅ Valid Address' : '❌ Invalid Address'}
              </Text>
            </Surface>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

// Search Demo Tab - Search functionality
function SearchDemo() {
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<SearchResult | null>(null);
  const [searchLoading, setSearchLoading] = useState(false);

  const sdk = NigeriaGeoSDK.getInstance();

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;

    setSearchLoading(true);
    try {
      const results = await sdk.search(searchQuery);
      setSearchResults(results);
    } catch (error) {
      Alert.alert('Search Error', error instanceof Error ? error.message : 'Search failed');
    } finally {
      setSearchLoading(false);
    }
  };

  return (
    <View style={styles.tabContent}>
      <Card style={styles.card}>
        <Card.Title title="Search Demo" subtitle="Search across all geographical data" />
        <Card.Content>
          <TextInput
            label="Search location"
            value={searchQuery}
            onChangeText={setSearchQuery}
            right={
              <TextInput.Icon
                icon="magnify"
                onPress={handleSearch}
                disabled={searchLoading}
              />
            }
            style={styles.searchInput}
          />

          <Button
            mode="contained"
            onPress={handleSearch}
            loading={searchLoading}
            disabled={searchLoading || !searchQuery.trim()}
            style={styles.fullButton}
          >
            Search
          </Button>

          {searchResults && (
            <>
              <Divider style={styles.divider} />
              <Text variant="titleMedium">Search Results:</Text>

              {searchResults.states.length > 0 && (
                <>
                  <Text variant="titleSmall" style={styles.resultTitle}>States ({searchResults.states.length})</Text>
                  {searchResults.states.slice(0, 3).map((state) => (
                    <Chip key={state.id} style={styles.chip} icon="map-marker">
                      {state.name} - {state.capital}
                    </Chip>
                  ))}
                </>
              )}

              {searchResults.lgas.length > 0 && (
                <>
                  <Text variant="titleSmall" style={styles.resultTitle}>LGAs ({searchResults.lgas.length})</Text>
                  {searchResults.lgas.slice(0, 3).map((lga) => (
                    <Chip key={lga.id} style={styles.chip} icon="city">
                      {lga.name}
                    </Chip>
                  ))}
                </>
              )}

              {searchResults.wards.length > 0 && (
                <>
                  <Text variant="titleSmall" style={styles.resultTitle}>Wards ({searchResults.wards.length})</Text>
                  {searchResults.wards.slice(0, 3).map((ward) => (
                    <Chip key={ward.id} style={styles.chip} icon="home-city">
                      {ward.name}
                    </Chip>
                  ))}
                </>
              )}

              {searchResults.postal_codes.length > 0 && (
                <>
                  <Text variant="titleSmall" style={styles.resultTitle}>Postal Codes ({searchResults.postal_codes.length})</Text>
                  {searchResults.postal_codes.slice(0, 3).map((postal) => (
                    <Chip key={postal.id} style={styles.chip} icon="mailbox">
                      {postal.code} - {postal.area}
                    </Chip>
                  ))}
                </>
              )}
            </>
          )}
        </Card.Content>
      </Card>
    </View>
  );
}

const styles = StyleSheet.create({
  header: {
    backgroundColor: '#4caf50',
  },
  headerTitle: {
    color: 'white',
    fontSize: 20,
    fontWeight: 'bold',
  },
  tabContainer: {
    backgroundColor: '#4caf50',
    paddingHorizontal: 16,
    paddingVertical: 8,
  },
  segmentedButtons: {
    backgroundColor: 'rgba(255, 255, 255, 0.1)',
  },
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  tabContent: {
    padding: 16,
  },
  card: {
    marginBottom: 16,
  },
  buttonRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: 8,
  },
  apiButton: {
    flex: 1,
    marginHorizontal: 4,
  },
  fullButton: {
    marginTop: 12,
  },
  loadingContainer: {
    alignItems: 'center',
    padding: 20,
  },
  loadingText: {
    marginTop: 8,
  },
  outputContainer: {
    backgroundColor: '#f8f8f8',
    padding: 12,
    borderRadius: 8,
    maxHeight: 300,
  },
  outputText: {
    fontFamily: 'monospace',
    fontSize: 12,
    color: '#333',
  },
  chip: {
    margin: 4,
  },
  searchInput: {
    marginBottom: 8,
  },
  divider: {
    marginVertical: 12,
  },
  resultTitle: {
    marginTop: 8,
    marginBottom: 4,
    fontWeight: 'bold',
  },
  validationResult: {
    marginTop: 12,
    padding: 12,
    borderRadius: 8,
    alignItems: 'center',
  },
  validResult: {
    backgroundColor: '#e8f5e8',
  },
  invalidResult: {
    backgroundColor: '#ffeaa7',
  },
  validationText: {
    fontWeight: 'bold',
    fontSize: 16,
  },
});
