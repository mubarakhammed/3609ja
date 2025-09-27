/**
 * Nigeria Geo SDK - React Native
 * Zero-config SDK for Nigerian geographical data
 * 
 * @example
 * ```typescript
 * import { NigeriaGeoSDK, StateDropdown } from 'nigeria-geo-sdk';
 * 
 * // Zero-config usage
 * const states = await NigeriaGeoSDK.getStates();
 * 
 * // Custom base URL
 * const sdk = NigeriaGeoSDK.getInstance('https://your-api.com');
 * ```
 */

// Core SDK
export { NigeriaGeoSDK } from './src/NigeriaGeoSDK';

// TypeScript Types
export type {
    State,
    Lga,
    Ward,
    PostalCode,
    ApiResponse,
    SearchResult,
    Geographic,
    ApiError
} from './src/types';

// React Native UI Components
export {
    StateDropdown,
    LgaDropdown,
    WardDropdown,
    PostalCodeDropdown
} from './src/components';

// Default export for zero-config usage
import { NigeriaGeoSDK } from './src/NigeriaGeoSDK';
export default NigeriaGeoSDK;