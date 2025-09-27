/**
 * Nigeria Geo SDK - React Native UI Components
 * Ready-to-use components for geographical data selection
 */

import React, { useState, useEffect } from 'react';
import { View, StyleSheet } from 'react-native';
import { Picker } from '@react-native-picker/picker';
import { Text, Card, ActivityIndicator } from 'react-native-paper';
import { NigeriaGeoSDK } from './NigeriaGeoSDK';
import { State, Lga, Ward, PostalCode } from './types';

interface StateDropdownProps {
    onStateChange: (state: State | null) => void;
    selectedState?: State | null;
    placeholder?: string;
    style?: any;
}

export const StateDropdown: React.FC<StateDropdownProps> = ({
    onStateChange,
    selectedState,
    placeholder = "Select State",
    style
}) => {
    const [states, setStates] = useState<State[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        loadStates();
    }, []);

    const loadStates = async () => {
        setLoading(true);
        setError(null);
        try {
            const sdk = NigeriaGeoSDK.getInstance();
            const data = await sdk.getStates();
            setStates(data);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load states');
        } finally {
            setLoading(false);
        }
    };

    if (loading) {
        return (
            <View style={[styles.container, style]}>
                <ActivityIndicator size="small" />
                <Text>Loading states...</Text>
            </View>
        );
    }

    if (error) {
        return (
            <View style={[styles.container, style]}>
                <Text style={styles.errorText}>Error: {error}</Text>
            </View>
        );
    }

    return (
        <View style={[styles.container, style]}>
            <Picker
                selectedValue={selectedState?.id?.toString() || ''}
                onValueChange={(itemValue: string) => {
                    if (itemValue === '') {
                        onStateChange(null);
                    } else {
                        const state = states.find(s => s.id === parseInt(itemValue));
                        onStateChange(state || null);
                    }
                }}
                style={styles.picker}
            >
                <Picker.Item label={placeholder} value="" />
                {states.map((state) => (
                    <Picker.Item key={state.id} label={state.name} value={state.id.toString()} />
                ))}
            </Picker>
        </View>
    );
};

interface LgaDropdownProps {
    stateId?: number;
    onLgaChange: (lga: Lga | null) => void;
    selectedLga?: Lga | null;
    placeholder?: string;
    style?: any;
}

export const LgaDropdown: React.FC<LgaDropdownProps> = ({
    stateId,
    onLgaChange,
    selectedLga,
    placeholder = "Select LGA",
    style
}) => {
    const [lgas, setLgas] = useState<Lga[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (stateId) {
            loadLgas();
        } else {
            setLgas([]);
            onLgaChange(null);
        }
    }, [stateId]);

    const loadLgas = async () => {
        if (!stateId) return;

        setLoading(true);
        setError(null);
        try {
            const sdk = NigeriaGeoSDK.getInstance();
            const data = await sdk.getLgas(stateId);
            setLgas(data);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load LGAs');
        } finally {
            setLoading(false);
        }
    };

    const isDisabled = !stateId || loading;

    if (loading) {
        return (
            <View style={[styles.container, style]}>
                <ActivityIndicator size="small" />
                <Text>Loading LGAs...</Text>
            </View>
        );
    }

    return (
        <View style={[styles.container, style]}>
            <Picker
                selectedValue={selectedLga?.id?.toString() || ''}
                onValueChange={(itemValue: string) => {
                    if (itemValue === '') {
                        onLgaChange(null);
                    } else {
                        const lga = lgas.find(l => l.id === parseInt(itemValue));
                        onLgaChange(lga || null);
                    }
                }}
                style={[styles.picker, isDisabled && styles.disabledPicker]}
                enabled={!isDisabled}
            >
                <Picker.Item label={isDisabled ? "Select state first" : placeholder} value="" />
                {lgas.map((lga) => (
                    <Picker.Item key={lga.id} label={lga.name} value={lga.id.toString()} />
                ))}
            </Picker>
            {error && <Text style={styles.errorText}>Error: {error}</Text>}
        </View>
    );
};

interface WardDropdownProps {
    lgaId?: number;
    onWardChange: (ward: Ward | null) => void;
    selectedWard?: Ward | null;
    placeholder?: string;
    style?: any;
}

export const WardDropdown: React.FC<WardDropdownProps> = ({
    lgaId,
    onWardChange,
    selectedWard,
    placeholder = "Select Ward",
    style
}) => {
    const [wards, setWards] = useState<Ward[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (lgaId) {
            loadWards();
        } else {
            setWards([]);
            onWardChange(null);
        }
    }, [lgaId]);

    const loadWards = async () => {
        if (!lgaId) return;

        setLoading(true);
        setError(null);
        try {
            const sdk = NigeriaGeoSDK.getInstance();
            const data = await sdk.getWards(lgaId);
            setWards(data);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load wards');
        } finally {
            setLoading(false);
        }
    };

    const isDisabled = !lgaId || loading;

    if (loading) {
        return (
            <View style={[styles.container, style]}>
                <ActivityIndicator size="small" />
                <Text>Loading wards...</Text>
            </View>
        );
    }

    return (
        <View style={[styles.container, style]}>
            <Picker
                selectedValue={selectedWard?.id?.toString() || ''}
                onValueChange={(itemValue: string) => {
                    if (itemValue === '') {
                        onWardChange(null);
                    } else {
                        const ward = wards.find(w => w.id === parseInt(itemValue));
                        onWardChange(ward || null);
                    }
                }}
                style={[styles.picker, isDisabled && styles.disabledPicker]}
                enabled={!isDisabled}
            >
                <Picker.Item label={isDisabled ? "Select LGA first" : placeholder} value="" />
                {wards.map((ward) => (
                    <Picker.Item key={ward.id} label={ward.name} value={ward.id.toString()} />
                ))}
            </Picker>
            {error && <Text style={styles.errorText}>Error: {error}</Text>}
        </View>
    );
};

interface PostalCodeDropdownProps {
    stateId?: number;
    onPostalCodeChange: (postalCode: PostalCode | null) => void;
    selectedPostalCode?: PostalCode | null;
    placeholder?: string;
    style?: any;
}

export const PostalCodeDropdown: React.FC<PostalCodeDropdownProps> = ({
    stateId,
    onPostalCodeChange,
    selectedPostalCode,
    placeholder = "Select Postal Code",
    style
}) => {
    const [postalCodes, setPostalCodes] = useState<PostalCode[]>([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (stateId) {
            loadPostalCodes();
        } else {
            setPostalCodes([]);
            onPostalCodeChange(null);
        }
    }, [stateId]);

    const loadPostalCodes = async () => {
        if (!stateId) return;

        setLoading(true);
        setError(null);
        try {
            const sdk = NigeriaGeoSDK.getInstance();
            const data = await sdk.getPostalCodes(stateId);
            setPostalCodes(data);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to load postal codes');
        } finally {
            setLoading(false);
        }
    };

    const isDisabled = !stateId || loading;

    if (loading) {
        return (
            <View style={[styles.container, style]}>
                <ActivityIndicator size="small" />
                <Text>Loading postal codes...</Text>
            </View>
        );
    }

    return (
        <View style={[styles.container, style]}>
            <Picker
                selectedValue={selectedPostalCode?.id?.toString() || ''}
                onValueChange={(itemValue: string) => {
                    if (itemValue === '') {
                        onPostalCodeChange(null);
                    } else {
                        const postalCode = postalCodes.find(p => p.id === parseInt(itemValue));
                        onPostalCodeChange(postalCode || null);
                    }
                }}
                style={[styles.picker, isDisabled && styles.disabledPicker]}
                enabled={!isDisabled}
            >
                <Picker.Item label={isDisabled ? "Select state first" : placeholder} value="" />
                {postalCodes.map((postalCode) => (
                    <Picker.Item
                        key={postalCode.id}
                        label={`${postalCode.code} - ${postalCode.area}`}
                        value={postalCode.id.toString()}
                    />
                ))}
            </Picker>
            {error && <Text style={styles.errorText}>Error: {error}</Text>}
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        marginVertical: 8,
    },
    picker: {
        backgroundColor: '#f5f5f5',
        borderRadius: 8,
    },
    disabledPicker: {
        opacity: 0.5,
    },
    errorText: {
        color: '#d32f2f',
        fontSize: 12,
        marginTop: 4,
    },
});