import ConfigLoader from './config';

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
    private config: Config = {} as Config;

    private constructor(config?: Partial<Config>, environment?: string) {
        // Load configuration from file with fallback to provided config
        this.initializeConfig(config, environment);
    }

    private async initializeConfig(config?: Partial<Config>, environment?: string): Promise<void> {
        try {
            const loadedConfig = await ConfigLoader.loadConfig(environment);
            this.config = {
                ...loadedConfig,
                ...config // Override with any provided config
            };
        } catch (error) {
            console.warn('Failed to load config, using optimized defaults:', error);
            this.config = {
                baseUrl: 'http://20.63.52.179:3000', // Use production server as fallback
                timeout: 5000, // Optimized timeout for fast API
                enableCaching: true,
                enableLogging: false, // Disable logging by default
                ...config
            };
        }
    }

    static getInstance(config?: Partial<Config>, environment?: string): NigeriaGeoSDK {
        if (!NigeriaGeoSDK.instance) {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(config, environment);
        }
        return NigeriaGeoSDK.instance;
    }

    // Method to reconfigure the SDK
    static async reconfigure(config?: Partial<Config>, environment?: string): Promise<NigeriaGeoSDK> {
        if (NigeriaGeoSDK.instance) {
            await NigeriaGeoSDK.instance.initializeConfig(config, environment);
        } else {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(config, environment);
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

    // States API - Enhanced with optional pagination
    async getStates(limit: number = 50): Promise<State[]> {
        const response = await this.makeRequest<ApiResponse<State>>(`/api/v1/states?limit=${limit}`);
        return response.data;
    }

    async getState(id: string): Promise<State> {
        const response = await this.makeRequest<State>(`/api/v1/states/${id}`);
        return response;
    }

    // LGAs API - Enhanced with optional pagination
    async getLgasByState(stateId: string, limit: number = 50): Promise<Lga[]> {
        const response = await this.makeRequest<ApiResponse<Lga>>(`/api/v1/states/${stateId}/lgas?limit=${limit}`);
        return response.data;
    }

    async getLga(id: string): Promise<Lga> {
        const response = await this.makeRequest<Lga>(`/api/v1/lgas/${id}`);
        return response;
    }

    // Wards API - Enhanced with optional pagination
    async getWardsByLga(lgaId: string, limit: number = 100): Promise<Ward[]> {
        const response = await this.makeRequest<ApiResponse<Ward>>(`/api/v1/lgas/${lgaId}/wards?limit=${limit}`);
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