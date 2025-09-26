#!/usr/bin/env python3
"""
Generate OpenAPI JSON specification for Nigeria Geo API
This script creates a standalone OpenAPI JSON file that can be imported into API clients.
"""

import json
import sys
import os

# Add the project root to the Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

def generate_openapi_json():
    """Generate the OpenAPI JSON specification"""
    
    # This is a comprehensive OpenAPI 3.0 specification for the Nigeria Geo API
    openapi_spec = {
        "openapi": "3.0.3",
        "info": {
            "title": "Nigeria Geo + Postal + Validation API",
            "description": "A comprehensive API for Nigerian geographic data, postal codes, and address validation",
            "version": "1.0.0",
            "contact": {
                "name": "Nigeria Geo API",
                "email": "support@nigeriageoapi.com"
            },
            "license": {
                "name": "MIT",
                "url": "https://opensource.org/licenses/MIT"
            }
        },
        "servers": [
            {
                "url": "http://localhost:3000",
                "description": "Development server"
            },
            {
                "url": "https://api.nigeriageo.com",
                "description": "Production server"
            }
        ],
        "tags": [
            {
                "name": "States",
                "description": "Nigerian states management"
            },
            {
                "name": "LGAs",
                "description": "Local Government Areas management"
            },
            {
                "name": "Wards",
                "description": "Wards management"
            },
            {
                "name": "Postal Codes",
                "description": "Postal codes and geographic data"
            },
            {
                "name": "Address Validation",
                "description": "Address validation and suggestions"
            },
            {
                "name": "Search",
                "description": "Search across all geographic entities"
            }
        ],
        "paths": {
            "/api/v1/states": {
                "get": {
                    "tags": ["States"],
                    "summary": "Get all states with pagination",
                    "parameters": [
                        {
                            "name": "page",
                            "in": "query",
                            "description": "Page number (1-based)",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "example": 1
                            }
                        },
                        {
                            "name": "limit",
                            "in": "query",
                            "description": "Number of items per page",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "maximum": 100,
                                "example": 20
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of states",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/PaginatedResponseStateDto"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Bad request"
                        },
                        "500": {
                            "description": "Internal server error"
                        }
                    }
                }
            },
            "/api/v1/states/{id}": {
                "get": {
                    "tags": ["States"],
                    "summary": "Get state by ID",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": True,
                            "description": "State ID",
                            "schema": {
                                "type": "string",
                                "format": "uuid"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "State found",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/StateDto"
                                    }
                                }
                            }
                        },
                        "404": {
                            "description": "State not found"
                        },
                        "500": {
                            "description": "Internal server error"
                        }
                    }
                }
            },
            "/api/v1/states/{id}/lgas": {
                "get": {
                    "tags": ["States"],
                    "summary": "Get LGAs by state ID",
                    "parameters": [
                        {
                            "name": "id",
                            "in": "path",
                            "required": True,
                            "description": "State ID",
                            "schema": {
                                "type": "string",
                                "format": "uuid"
                            }
                        },
                        {
                            "name": "page",
                            "in": "query",
                            "description": "Page number (1-based)",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "example": 1
                            }
                        },
                        {
                            "name": "limit",
                            "in": "query",
                            "description": "Number of items per page",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "maximum": 100,
                                "example": 20
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "List of LGAs",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/PaginatedResponseLgaDto"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/api/v1/validate": {
                "post": {
                    "tags": ["Address Validation"],
                    "summary": "Validate a Nigerian address",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/AddressValidationRequestDto"
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Address validation result",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/AddressValidationResponseDto"
                                    }
                                }
                            }
                        },
                        "400": {
                            "description": "Bad request"
                        },
                        "500": {
                            "description": "Internal server error"
                        }
                    }
                }
            },
            "/api/v1/search": {
                "get": {
                    "tags": ["Search"],
                    "summary": "Search across all entities",
                    "parameters": [
                        {
                            "name": "query",
                            "in": "query",
                            "required": True,
                            "description": "Search query",
                            "schema": {
                                "type": "string",
                                "example": "Lagos"
                            }
                        },
                        {
                            "name": "page",
                            "in": "query",
                            "description": "Page number (1-based)",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "example": 1
                            }
                        },
                        {
                            "name": "limit",
                            "in": "query",
                            "description": "Number of items per page",
                            "required": False,
                            "schema": {
                                "type": "integer",
                                "minimum": 1,
                                "maximum": 100,
                                "example": 20
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "Search results",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/SearchResultDto"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "StateDto": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "Unique identifier for the state",
                            "example": "550e8400-e29b-41d4-a716-446655440001"
                        },
                        "name": {
                            "type": "string",
                            "description": "Name of the state",
                            "example": "Lagos"
                        },
                        "code": {
                            "type": "string",
                            "description": "State code (e.g., NG-LA)",
                            "example": "NG-LA"
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Creation timestamp"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Last update timestamp"
                        }
                    },
                    "required": ["id", "name", "code", "created_at", "updated_at"]
                },
                "LgaDto": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "Unique identifier for the LGA",
                            "example": "650e8400-e29b-41d4-a716-446655440001"
                        },
                        "state_id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "ID of the parent state",
                            "example": "550e8400-e29b-41d4-a716-446655440001"
                        },
                        "name": {
                            "type": "string",
                            "description": "Name of the LGA",
                            "example": "Ikeja"
                        },
                        "code": {
                            "type": "string",
                            "description": "LGA code (e.g., NG-LA-01)",
                            "example": "NG-LA-01"
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Creation timestamp"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Last update timestamp"
                        }
                    },
                    "required": ["id", "state_id", "name", "code", "created_at", "updated_at"]
                },
                "WardDto": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "Unique identifier for the ward",
                            "example": "750e8400-e29b-41d4-a716-446655440001"
                        },
                        "lga_id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "ID of the parent LGA",
                            "example": "650e8400-e29b-41d4-a716-446655440001"
                        },
                        "name": {
                            "type": "string",
                            "description": "Name of the ward",
                            "example": "Ikeja"
                        },
                        "code": {
                            "type": "string",
                            "description": "Ward code (e.g., NG-LA-01-01)",
                            "example": "NG-LA-01-01"
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Creation timestamp"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Last update timestamp"
                        }
                    },
                    "required": ["id", "lga_id", "name", "code", "created_at", "updated_at"]
                },
                "PostalCodeDto": {
                    "type": "object",
                    "properties": {
                        "id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "Unique identifier for the postal code",
                            "example": "850e8400-e29b-41d4-a716-446655440001"
                        },
                        "ward_id": {
                            "type": "string",
                            "format": "uuid",
                            "description": "ID of the parent ward",
                            "example": "750e8400-e29b-41d4-a716-446655440001"
                        },
                        "postal_code": {
                            "type": "string",
                            "description": "Postal code",
                            "example": "100001"
                        },
                        "lat": {
                            "type": "number",
                            "format": "double",
                            "description": "Latitude coordinate",
                            "example": 6.6059
                        },
                        "lng": {
                            "type": "number",
                            "format": "double",
                            "description": "Longitude coordinate",
                            "example": 3.3515
                        },
                        "urban": {
                            "type": "boolean",
                            "description": "Whether this is an urban area",
                            "example": True
                        },
                        "created_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Creation timestamp"
                        },
                        "updated_at": {
                            "type": "string",
                            "format": "date-time",
                            "description": "Last update timestamp"
                        }
                    },
                    "required": ["id", "ward_id", "postal_code", "urban", "created_at", "updated_at"]
                },
                "AddressDto": {
                    "type": "object",
                    "properties": {
                        "state": {
                            "$ref": "#/components/schemas/StateDto"
                        },
                        "lga": {
                            "$ref": "#/components/schemas/LgaDto"
                        },
                        "ward": {
                            "$ref": "#/components/schemas/WardDto"
                        },
                        "postal_code": {
                            "$ref": "#/components/schemas/PostalCodeDto"
                        }
                    },
                    "required": ["state", "lga", "ward", "postal_code"]
                },
                "AddressValidationRequestDto": {
                    "type": "object",
                    "properties": {
                        "state": {
                            "type": "string",
                            "description": "State name",
                            "example": "Lagos"
                        },
                        "lga": {
                            "type": "string",
                            "description": "Local Government Area name",
                            "example": "Ikeja"
                        },
                        "ward": {
                            "type": "string",
                            "description": "Ward name",
                            "example": "Ikeja"
                        },
                        "postal_code": {
                            "type": "string",
                            "description": "Postal code",
                            "example": "100001"
                        }
                    },
                    "required": ["state", "lga", "ward", "postal_code"]
                },
                "AddressValidationResponseDto": {
                    "type": "object",
                    "properties": {
                        "valid": {
                            "type": "boolean",
                            "description": "Whether the address is valid",
                            "example": True
                        },
                        "canonical": {
                            "$ref": "#/components/schemas/AddressDto"
                        },
                        "suggestions": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/AddressSuggestionDto"
                            },
                            "description": "Suggested corrections if invalid"
                        }
                    },
                    "required": ["valid", "suggestions"]
                },
                "AddressSuggestionDto": {
                    "type": "object",
                    "properties": {
                        "state": {
                            "$ref": "#/components/schemas/StateDto"
                        },
                        "lga": {
                            "$ref": "#/components/schemas/LgaDto"
                        },
                        "ward": {
                            "$ref": "#/components/schemas/WardDto"
                        },
                        "postal_code": {
                            "$ref": "#/components/schemas/PostalCodeDto"
                        },
                        "reason": {
                            "type": "string",
                            "description": "Reason for suggestion",
                            "example": "Similar name found"
                        },
                        "confidence": {
                            "type": "number",
                            "format": "double",
                            "description": "Confidence score (0.0 to 1.0)",
                            "minimum": 0.0,
                            "maximum": 1.0,
                            "example": 0.85
                        }
                    },
                    "required": ["reason", "confidence"]
                },
                "PaginationMeta": {
                    "type": "object",
                    "properties": {
                        "page": {
                            "type": "integer",
                            "description": "Current page number",
                            "example": 1
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Items per page",
                            "example": 20
                        },
                        "total": {
                            "type": "integer",
                            "format": "int64",
                            "description": "Total number of items",
                            "example": 100
                        },
                        "total_pages": {
                            "type": "integer",
                            "description": "Total number of pages",
                            "example": 5
                        },
                        "has_next": {
                            "type": "boolean",
                            "description": "Whether there is a next page",
                            "example": True
                        },
                        "has_prev": {
                            "type": "boolean",
                            "description": "Whether there is a previous page",
                            "example": False
                        }
                    },
                    "required": ["page", "limit", "total", "total_pages", "has_next", "has_prev"]
                },
                "PaginatedResponseStateDto": {
                    "type": "object",
                    "properties": {
                        "data": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/StateDto"
                            },
                            "description": "Array of data items"
                        },
                        "pagination": {
                            "$ref": "#/components/schemas/PaginationMeta"
                        }
                    },
                    "required": ["data", "pagination"]
                },
                "PaginatedResponseLgaDto": {
                    "type": "object",
                    "properties": {
                        "data": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/LgaDto"
                            },
                            "description": "Array of data items"
                        },
                        "pagination": {
                            "$ref": "#/components/schemas/PaginationMeta"
                        }
                    },
                    "required": ["data", "pagination"]
                },
                "SearchResultDto": {
                    "type": "object",
                    "properties": {
                        "states": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/StateDto"
                            }
                        },
                        "lgas": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/LgaDto"
                            }
                        },
                        "wards": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/WardDto"
                            }
                        },
                        "postal_codes": {
                            "type": "array",
                            "items": {
                                "$ref": "#/components/schemas/PostalCodeDto"
                            }
                        }
                    },
                    "required": ["states", "lgas", "wards", "postal_codes"]
                }
            }
        }
    }
    
    return openapi_spec

def main():
    """Main function to generate and save the OpenAPI JSON"""
    try:
        # Generate the OpenAPI specification
        openapi_spec = generate_openapi_json()
        
        # Save to file
        output_file = os.path.join(os.path.dirname(__file__), "..", "openapi.json")
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(openapi_spec, f, indent=2, ensure_ascii=False)
        
        print(f"✅ OpenAPI JSON specification generated successfully!")
        print(f"📄 File saved to: {output_file}")
        print(f"🔗 You can now import this file into:")
        print(f"   - Postman (Import → Link)")
        print(f"   - Insomnia (Create → Import from File)")
        print(f"   - Swagger Editor (File → Import File)")
        print(f"   - Any other OpenAPI-compatible tool")
        
    except Exception as e:
        print(f"❌ Error generating OpenAPI JSON: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
