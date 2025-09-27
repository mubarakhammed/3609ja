/**
 * Nigeria Geo SDK - Core API Client
 * Zero-config SDK for Nigerian geographical data
 */

import { State, Lga, Ward, PostalCode, ApiResponse, SearchResult } from './types';

export class NigeriaGeoSDK {
    private static instance: NigeriaGeoSDK;
    private baseUrl: string;

    private constructor(baseUrl?: string) {
        // Zero-config: Use the local development API by default
        this.baseUrl = baseUrl || 'http://localhost:3000';
    }

    /**
     * Get singleton instance with zero configuration
     */
    public static getInstance(baseUrl?: string): NigeriaGeoSDK {
        if (!NigeriaGeoSDK.instance) {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(baseUrl);
        }
        return NigeriaGeoSDK.instance;
    }

    /**
     * Make HTTP request with error handling
     */
    private async makeRequest<T>(endpoint: string): Promise<T> {
        try {
            const response = await fetch(`${this.baseUrl}${endpoint}`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json',
                },
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const data = await response.json();
            return data;
        } catch (error) {
            if (error instanceof Error) {
                throw new ApiError(error.message);
            }
            throw new ApiError('Unknown error occurred');
        }
    }

    // States API
    async getStates(): Promise<State[]> {
        const response = await this.makeRequest<ApiResponse<State>>('/api/v1/states');
        return response.data;
    }

    async getState(id: string): Promise<State> {
        const response = await this.makeRequest<State>(`/api/v1/states/${id}`);
        return response;
    }

    // LGAs API
    async getLgasByState(stateId: string): Promise<Lga[]> {
        const response = await this.makeRequest<ApiResponse<Lga>>(`/api/v1/states/${stateId}/lgas`);
        return response.data;
    }

    async getLga(id: string): Promise<Lga> {
        const response = await this.makeRequest<Lga>(`/api/v1/lgas/${id}`);
        return response;
    }

    // Wards API
    async getWardsByLga(lgaId: string): Promise<Ward[]> {
        const response = await this.makeRequest<ApiResponse<Ward>>(`/api/v1/lgas/${lgaId}/wards`);
        return response.data;
    }

    async getWard(id: string): Promise<Ward> {
        const response = await this.makeRequest<Ward>(`/api/v1/wards/${id}`);
        return response;
    }

    // Postal Codes API
    async getPostalCodesByState(stateId: string): Promise<PostalCode[]> {
        const response = await this.makeRequest<ApiResponse<PostalCode>>(`/api/v1/states/${stateId}/postal-codes`);
        return response.data;
    }

    async getPostalCode(id: string): Promise<PostalCode> {
        const response = await this.makeRequest<PostalCode>(`/api/v1/postal-codes/${id}`);
        return response;
    }

    // Search API
    async search(query: string): Promise<SearchResult> {
        const encodedQuery = encodeURIComponent(query);
        const response = await this.makeRequest<SearchResult>(`/api/v1/search?q=${encodedQuery}`);
        return response;
    }

    // Validation API
    async validateAddress(
        state?: string,
        lga?: string,
        ward?: string,
        postalCode?: string
    ): Promise<boolean> {
        const params = new URLSearchParams();
        if (state) params.append('state', state);
        if (lga) params.append('lga', lga);
        if (ward) params.append('ward', ward);
        if (postalCode) params.append('postal_code', postalCode);

        const response = await this.makeRequest<{ valid: boolean }>(`/api/v1/validate?${params.toString()}`);
        return response.valid;
    }
}

class ApiError extends Error {
    public code?: string;
    public status?: number;

    constructor(message: string, code?: string, status?: number) {
        super(message);
        this.name = 'ApiError';
        this.code = code;
        this.status = status;
    }
}

// Export default instance for zero-config usage
export default NigeriaGeoSDK.getInstance();