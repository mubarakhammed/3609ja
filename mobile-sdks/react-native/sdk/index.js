"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.NigeriaGeoSDK = exports.nigeriaGeoSDK = void 0;
const config_1 = __importDefault(require("./config"));
// Core SDK Class
class NigeriaGeoSDK {
    constructor(config, environment) {
        this.config = {};
        // Load configuration from file with fallback to provided config
        this.initializeConfig(config, environment);
    }
    async initializeConfig(config, environment) {
        try {
            const loadedConfig = await config_1.default.loadConfig(environment);
            this.config = {
                ...loadedConfig,
                ...config // Override with any provided config
            };
        }
        catch (error) {
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
    static getInstance(config, environment) {
        if (!NigeriaGeoSDK.instance) {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(config, environment);
        }
        return NigeriaGeoSDK.instance;
    }
    // Method to reconfigure the SDK
    static async reconfigure(config, environment) {
        if (NigeriaGeoSDK.instance) {
            await NigeriaGeoSDK.instance.initializeConfig(config, environment);
        }
        else {
            NigeriaGeoSDK.instance = new NigeriaGeoSDK(config, environment);
        }
        return NigeriaGeoSDK.instance;
    }
    async makeRequest(endpoint) {
        const url = `${this.config.baseUrl}${endpoint}`;
        try {
            // Create a timeout promise for React Native compatibility
            const timeoutPromise = new Promise((_, reject) => {
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
        }
        catch (error) {
            if (this.config.enableLogging) {
                console.error('Nigeria Geo SDK Error:', error);
            }
            throw error;
        }
    }
    // States API - Enhanced with optional pagination
    async getStates(limit = 50) {
        const response = await this.makeRequest(`/api/v1/states?limit=${limit}`);
        return response.data;
    }
    async getState(id) {
        const response = await this.makeRequest(`/api/v1/states/${id}`);
        return response;
    }
    // LGAs API - Enhanced with optional pagination
    async getLgasByState(stateId, limit = 50) {
        const response = await this.makeRequest(`/api/v1/states/${stateId}/lgas?limit=${limit}`);
        return response.data;
    }
    async getLga(id) {
        const response = await this.makeRequest(`/api/v1/lgas/${id}`);
        return response;
    }
    // Wards API - Enhanced with optional pagination
    async getWardsByLga(lgaId, limit = 100) {
        const response = await this.makeRequest(`/api/v1/lgas/${lgaId}/wards?limit=${limit}`);
        return response.data;
    }
    async getWard(id) {
        const response = await this.makeRequest(`/api/v1/wards/${id}`);
        return response;
    }
    // Postal Codes API
    async getPostalCodesByState(stateId) {
        const response = await this.makeRequest(`/api/v1/states/${stateId}/postal-codes`);
        return response.data;
    }
    async getPostalCode(id) {
        const response = await this.makeRequest(`/api/v1/postal-codes/${id}`);
        return response;
    }
    // Search API
    async search(query) {
        const encodedQuery = encodeURIComponent(query);
        const response = await this.makeRequest(`/api/v1/search?q=${encodedQuery}`);
        return response;
    }
}
exports.NigeriaGeoSDK = NigeriaGeoSDK;
// Export singleton instance for zero-config usage
exports.nigeriaGeoSDK = NigeriaGeoSDK.getInstance();
exports.default = exports.nigeriaGeoSDK;
