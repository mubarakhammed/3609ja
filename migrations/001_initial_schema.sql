-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create states table
CREATE TABLE states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL UNIQUE,
    code VARCHAR(10) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create LGAs table
CREATE TABLE lgas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    state_id UUID NOT NULL REFERENCES states(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    code VARCHAR(20) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(state_id, name)
);

-- Create wards table
CREATE TABLE wards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    lga_id UUID NOT NULL REFERENCES lgas(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    code VARCHAR(30) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(lga_id, name)
);

-- Create postal codes table
CREATE TABLE postal_codes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    ward_id UUID NOT NULL REFERENCES wards(id) ON DELETE CASCADE,
    postal_code VARCHAR(10) NOT NULL,
    lat DOUBLE PRECISION,
    lng DOUBLE PRECISION,
    urban BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(ward_id, postal_code)
);

-- Create indexes for better performance
CREATE INDEX idx_states_name ON states(name);
CREATE INDEX idx_states_code ON states(code);

CREATE INDEX idx_lgas_state_id ON lgas(state_id);
CREATE INDEX idx_lgas_name ON lgas(name);
CREATE INDEX idx_lgas_code ON lgas(code);

CREATE INDEX idx_wards_lga_id ON wards(lga_id);
CREATE INDEX idx_wards_name ON wards(name);
CREATE INDEX idx_wards_code ON wards(code);

CREATE INDEX idx_postal_codes_ward_id ON postal_codes(ward_id);
CREATE INDEX idx_postal_codes_code ON postal_codes(postal_code);
CREATE INDEX idx_postal_codes_coordinates ON postal_codes(lat, lng) WHERE lat IS NOT NULL AND lng IS NOT NULL;

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at
CREATE TRIGGER update_states_updated_at BEFORE UPDATE ON states
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_lgas_updated_at BEFORE UPDATE ON lgas
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_wards_updated_at BEFORE UPDATE ON wards
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_postal_codes_updated_at BEFORE UPDATE ON postal_codes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();