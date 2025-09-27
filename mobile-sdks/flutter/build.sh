#!/bin/bash

# Flutter SDK Build Script
echo "🛠️ Building Nigeria Geo Flutter SDK..."

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo "❌ Flutter is not installed. Please install Flutter first."
    exit 1
fi

# Navigate to the Flutter SDK directory
cd "$(dirname "$0")"

echo "📁 Current directory: $(pwd)"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
flutter clean

# Get dependencies
echo "📦 Getting dependencies..."
flutter pub get

# Run code generation (if needed in the future)
# echo "⚙️ Running code generation..."
# flutter packages pub run build_runner build --delete-conflicting-outputs

# Analyze code
echo "🔍 Analyzing code..."
flutter analyze

# Run tests
echo "🧪 Running tests..."
flutter test

# Check formatting
echo "📝 Checking code formatting..."
flutter format --set-exit-if-changed lib/ test/

echo "✅ Build completed successfully!"
echo ""
echo "🚀 SDK is ready for use!"
echo "📖 Check the README.md for usage instructions."