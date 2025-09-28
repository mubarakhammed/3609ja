import { Config } from './index';
declare class ConfigLoader {
    private static config;
    static loadConfig(environment?: string): Promise<Config>;
    static getConfig(): Config | null;
    static resetConfig(): void;
}
export default ConfigLoader;
