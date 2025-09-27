// TypeScript Types
export interface State {
    id: string;
    name: string;
    code: string;
    created_at: string;
    updated_at: string;
}

export interface Lga {
    id: string;
    name: string;
    code: string;
    state_id: string;
    created_at: string;
    updated_at: string;
}

export interface Ward {
    id: string;
    name: string;
    code: string;
    lga_id: string;
    created_at: string;
    updated_at: string;
}

export interface PostalCode {
    id: string;
    code: string;
    state_id: string;
    created_at: string;
    updated_at: string;
}

export interface ApiResponse<T> {
    data: T[];
    pagination: {
        page: number;
        limit: number;
        total: number;
        total_pages: number;
        has_next: boolean;
        has_prev: boolean;
    };
}

export interface SearchResult {
    states: State[];
    lgas: Lga[];
    wards: Ward[];
    postal_codes: PostalCode[];
}

export interface Config {
    baseUrl: string;
    timeout: number;
    enableCaching: boolean;
    enableLogging: boolean;
}

// Core SDK Class
class NigeriaGeoSDK {
    private static instance: NigeriaGeoSDK;
    private config: Config;

    private constructor(config?: Partial<Config>) {
        // Zero-config defaults matching Flutter SDK
        this.config = {
            baseUrl: 'http://localhost:3000',
            timeout: 30000,
            enableCaching: true,
            enableLogging: true,
            ...config
        };
    }

    static getInstance(config?: Partial<Config>): NigeriaGeoSDK {
        if (!NigeriaGeoSDK.instance) {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(config);
        }
        return NigeriaGeoSDK.instance;
    }

    private async makeRequest<T>(endpoint: string): Promise<T> {
        const url = `${this.config.baseUrl}${endpoint}`;

        try {
            // Create a timeout promise for React Native compatibility
            const timeoutPromise = new Promise<never>((_, reject) => {
                setTimeout(() => reject(new Error('Request timeout')), this.config.timeout);
            });

            const fetchPromise = fetch(url, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            });

            const response = await Promise.race([fetchPromise, timeoutPromise]);

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            return await response.json();
        } catch (error) {
            if (this.config.enableLogging) {
                console.error('Nigeria Geo SDK Error:', error);
            }
            throw error;
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
}

// Export singleton instance for zero-config usage
export const nigeriaGeoSDK = NigeriaGeoSDK.getInstance();
export { NigeriaGeoSDK };
export default nigeriaGeoSDK;