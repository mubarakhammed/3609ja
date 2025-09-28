const { getDefaultConfig } = require('expo/metro-config');
const path = require('path');

const config = getDefaultConfig(__dirname);

// Add the SDK source directory to watchFolders
config.watchFolders = [
    path.resolve(__dirname, '../sdk'),
];

// Add TypeScript support for the SDK
config.resolver.sourceExts = [...config.resolver.sourceExts, 'ts', 'tsx'];

module.exports = config;