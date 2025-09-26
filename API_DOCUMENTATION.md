# 📚 Nigeria Geo API Documentation

## 🎯 **OpenAPI/Swagger Documentation**

The Nigeria Geo API now includes comprehensive OpenAPI 3.0 documentation that can be imported into various API clients and tools.

### 📄 **Available Documentation Formats**

#### 1. **Swagger UI (Interactive)**
- **URL**: `http://localhost:3000/docs`
- **Description**: Interactive web-based API documentation
- **Features**: 
  - Try out endpoints directly in the browser
  - View request/response schemas
  - Test API calls with real data

#### 2. **OpenAPI JSON (Importable)**
- **URL**: `http://localhost:3000/api-docs/openapi.json`
- **File**: `openapi.json` (in project root)
- **Description**: Raw OpenAPI 3.0 specification in JSON format

### 🔧 **How to Import the API Documentation**

#### **Postman**
1. Open Postman
2. Click **Import** → **Link**
3. Enter: `http://localhost:3000/api-docs/openapi.json`
4. Or import the local `openapi.json` file

#### **Insomnia**
1. Open Insomnia
2. Click **Create** → **Import from File**
3. Select the `openapi.json` file
4. Or use the URL: `http://localhost:3000/api-docs/openapi.json`

#### **Swagger Editor**
1. Go to [editor.swagger.io](https://editor.swagger.io)
2. Click **File** → **Import File**
3. Select the `openapi.json` file

#### **VS Code (REST Client)**
1. Install the "REST Client" extension
2. Import the OpenAPI spec
3. Generate HTTP requests automatically

### 📋 **API Endpoints Overview**

#### **States**
- `GET /api/v1/states` - List all states (paginated)
- `GET /api/v1/states/{id}` - Get state by ID
- `GET /api/v1/states/{id}/lgas` - Get LGAs by state

#### **LGAs (Local Government Areas)**
- `GET /api/v1/lgas/{id}` - Get LGA by ID
- `GET /api/v1/lgas/{id}/wards` - Get wards by LGA

#### **Wards**
- `GET /api/v1/wards/{id}` - Get ward by ID
- `GET /api/v1/wards/{id}/postal-codes` - Get postal codes by ward

#### **Postal Codes**
- `GET /api/v1/postal-codes/{id}` - Get postal code by ID
- `GET /api/v1/postal-codes/code/{code}` - Get postal code by code
- `GET /api/v1/postal-codes/nearby` - Find nearby postal codes

#### **Address Validation**
- `POST /api/v1/validate` - Validate Nigerian address
- `GET /api/v1/address/find` - Find address by components
- `POST /api/v1/address/similar` - Find similar addresses

#### **Search**
- `GET /api/v1/search` - Search across all entities
- `GET /api/v1/search/states` - Search states only
- `GET /api/v1/search/lgas` - Search LGAs only
- `GET /api/v1/search/wards` - Search wards only
- `GET /api/v1/search/postal-codes` - Search postal codes only

### 🏗️ **Data Models**

#### **StateDto**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "Lagos",
  "code": "NG-LA",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### **LgaDto**
```json
{
  "id": "650e8400-e29b-41d4-a716-446655440001",
  "state_id": "550e8400-e29b-41d4-a716-446655440001",
  "name": "Ikeja",
  "code": "NG-LA-01",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### **WardDto**
```json
{
  "id": "750e8400-e29b-41d4-a716-446655440001",
  "lga_id": "650e8400-e29b-41d4-a716-446655440001",
  "name": "Ikeja",
  "code": "NG-LA-01-01",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### **PostalCodeDto**
```json
{
  "id": "850e8400-e29b-41d4-a716-446655440001",
  "ward_id": "750e8400-e29b-41d4-a716-446655440001",
  "postal_code": "100001",
  "lat": 6.6059,
  "lng": 3.3515,
  "urban": true,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

#### **AddressValidationRequestDto**
```json
{
  "state": "Lagos",
  "lga": "Ikeja",
  "ward": "Ikeja",
  "postal_code": "100001"
}
```

#### **AddressValidationResponseDto**
```json
{
  "valid": true,
  "canonical": {
    "state": { /* StateDto */ },
    "lga": { /* LgaDto */ },
    "ward": { /* WardDto */ },
    "postal_code": { /* PostalCodeDto */ }
  },
  "suggestions": [
    {
      "state": { /* StateDto */ },
      "lga": { /* LgaDto */ },
      "ward": { /* WardDto */ },
      "postal_code": { /* PostalCodeDto */ },
      "reason": "Similar name found",
      "confidence": 0.85
    }
  ]
}
```

### 🔍 **Pagination**

All list endpoints support pagination:

```json
{
  "data": [/* array of items */],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 100,
    "total_pages": 5,
    "has_next": true,
    "has_prev": false
  }
}
```

**Query Parameters:**
- `page` (optional): Page number (1-based, default: 1)
- `limit` (optional): Items per page (1-100, default: 20)

### 🌍 **Geographic Search**

#### **Find Nearby Postal Codes**
```
GET /api/v1/postal-codes/nearby?lat=6.6059&lng=3.3515&radius_km=10.0
```

**Parameters:**
- `lat` (required): Latitude
- `lng` (required): Longitude  
- `radius_km` (optional): Search radius in kilometers (default: 10.0)

### 🔍 **Search Functionality**

#### **Global Search**
```
GET /api/v1/search?query=Lagos&page=1&limit=20
```

#### **Entity-Specific Search**
```
GET /api/v1/search/states?query=Lagos
GET /api/v1/search/lgas?query=Ikeja
GET /api/v1/search/wards?query=Ikeja
GET /api/v1/search/postal-codes?query=100001
```

### ✅ **Address Validation**

#### **Validate Address**
```bash
curl -X POST http://localhost:3000/api/v1/validate \
  -H "Content-Type: application/json" \
  -d '{
    "state": "Lagos",
    "lga": "Ikeja", 
    "ward": "Ikeja",
    "postal_code": "100001"
  }'
```

#### **Find Address by Components**
```
GET /api/v1/address/find?state=Lagos&lga=Ikeja&ward=Ikeja&postal_code=100001
```

### 🚀 **Getting Started**

1. **Start the server:**
   ```bash
   cargo run
   ```

2. **Access Swagger UI:**
   - Open: `http://localhost:3000/docs`

3. **Import into API client:**
   - Use: `http://localhost:3000/api-docs/openapi.json`
   - Or download: `openapi.json`

4. **Test the API:**
   ```bash
   # Get all states
   curl http://localhost:3000/api/v1/states
   
   # Search for Lagos
   curl http://localhost:3000/api/v1/search?query=Lagos
   
   # Validate an address
   curl -X POST http://localhost:3000/api/v1/validate \
     -H "Content-Type: application/json" \
     -d '{"state":"Lagos","lga":"Ikeja","ward":"Ikeja","postal_code":"100001"}'
   ```

### 📊 **Response Codes**

- `200` - Success
- `400` - Bad Request (invalid parameters)
- `404` - Not Found
- `500` - Internal Server Error

### 🔧 **Development**

The OpenAPI specification is automatically generated from Rust code using:
- `utoipa` - OpenAPI derive macros
- `utoipa-swagger-ui` - Swagger UI integration
- `serde` - JSON serialization

**Files:**
- `src/presentation/openapi.rs` - OpenAPI specification
- `src/presentation/handlers.rs` - Endpoint documentation
- `src/application/dtos/*.rs` - Schema definitions
- `openapi.json` - Generated JSON specification

---

## 🎉 **Ready to Use!**

Your Nigeria Geo API now has comprehensive documentation that can be imported into any OpenAPI-compatible tool. The interactive Swagger UI makes it easy to explore and test the API, while the JSON specification enables integration with your favorite API clients.
