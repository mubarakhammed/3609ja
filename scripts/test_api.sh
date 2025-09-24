#!/bin/bash

# Test script for Nigeria Geo API
BASE_URL="http://localhost:3000/api/v1"

echo "🧪 Testing Nigeria Geo API"
echo "=========================="

# Test 1: Get all states
echo "1. Testing GET /states"
curl -s "$BASE_URL/states" | jq '.' || echo "Failed to get states"

echo -e "\n"

# Test 2: Search for Lagos
echo "2. Testing search for 'Lagos'"
curl -s "$BASE_URL/search?query=Lagos" | jq '.' || echo "Failed to search"

echo -e "\n"

# Test 3: Validate an address
echo "3. Testing address validation"
curl -s -X POST "$BASE_URL/validate" \
  -H "Content-Type: application/json" \
  -d '{
    "state": "Lagos",
    "lga": "Ikeja",
    "ward": "Ikeja Central",
    "postal_code": "100001"
  }' | jq '.' || echo "Failed to validate address"

echo -e "\n"

# Test 4: Get states with pagination
echo "4. Testing pagination"
curl -s "$BASE_URL/states?page=1&limit=5" | jq '.' || echo "Failed to get paginated states"

echo -e "\n"
echo "✅ API tests completed!"
