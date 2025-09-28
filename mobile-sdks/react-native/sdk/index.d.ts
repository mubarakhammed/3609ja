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
declare class NigeriaGeoSDK {
    private static instance;
    private config;
    private constructor();
    private initializeConfig;
    static getInstance(config?: Partial<Config>, environment?: string): NigeriaGeoSDK;
    static reconfigure(config?: Partial<Config>, environment?: string): Promise<NigeriaGeoSDK>;
    private makeRequest;
    getStates(limit?: number): Promise<State[]>;
    getState(id: string): Promise<State>;
    getLgasByState(stateId: string, limit?: number): Promise<Lga[]>;
    getLga(id: string): Promise<Lga>;
    getWardsByLga(lgaId: string, limit?: number): Promise<Ward[]>;
    getWard(id: string): Promise<Ward>;
    getPostalCodesByState(stateId: string): Promise<PostalCode[]>;
    getPostalCode(id: string): Promise<PostalCode>;
    search(query: string): Promise<SearchResult>;
}
export declare const nigeriaGeoSDK: NigeriaGeoSDK;
export { NigeriaGeoSDK };
export default nigeriaGeoSDK;
