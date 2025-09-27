/**
 * Nigeria Geo SDK - TypeScript Types
 * Zero-config SDK for Nigerian geographical data
 */

export interface State {
    id: string;
    name: string;
    code: string;
    capital: string;
    region: string;
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
    area: string;
    district: string;
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

export interface Geographic {
    state?: State;
    lga?: Lga;
    ward?: Ward;
    postal_code?: PostalCode;
}

export interface ApiError {
    message: string;
    code?: string;
    status?: number;
}