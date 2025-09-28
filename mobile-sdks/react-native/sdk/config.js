"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
// Configuration loader for React Native SDK
class ConfigLoader {
    static async loadConfig(environment = 'production') {
        if (ConfigLoader.config) {
            return ConfigLoader.config;
        }
        try {
            // Optimized production configuration
            const config = {
                baseUrl: 'http://20.63.52.179:3000', // Optimized production API URL
                timeout: 5000, // Reduced timeout for fast API
                enableCaching: true,
                enableLogging: false, // Disable logging in production
            };
            ConfigLoader.config = config;
            return config;
        }
        catch (error) {
            console.warn('Failed to load config, using production defaults:', error);
            // Fallback to optimized production config
            ConfigLoader.config = {
                baseUrl: 'http://20.63.52.179:3000',
                timeout: 5000, // Optimized timeout
                enableCaching: true,
                enableLogging: false,
            };
            return ConfigLoader.config;
        }
    }
    static getConfig() {
        return ConfigLoader.config;
    }
    static resetConfig() {
        ConfigLoader.config = null;
    }
}
ConfigLoader.config = null;
exports.default = ConfigLoader;
