-- Sample Postal Codes for Ikeja Ward
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440001', '750e8400-e29b-41d4-a716-446655440001', '100001', 6.6059, 3.3515, true),
    ('850e8400-e29b-41d4-a716-446655440002', '750e8400-e29b-41d4-a716-446655440001', '100002', 6.6060, 3.3516, true),
    ('850e8400-e29b-41d4-a716-446655440003', '750e8400-e29b-41d4-a716-446655440001', '100003', 6.6061, 3.3517, true);

-- Sample Postal Codes for Ojodu Ward
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440004', '750e8400-e29b-41d4-a716-446655440002', '100211', 6.6500, 3.3800, true),
    ('850e8400-e29b-41d4-a716-446655440005', '750e8400-e29b-41d4-a716-446655440002', '100212', 6.6501, 3.3801, true),
    ('850e8400-e29b-41d4-a716-446655440006', '750e8400-e29b-41d4-a716-446655440002', '100213', 6.6502, 3.3802, true);

-- Sample Postal Codes for Victoria Island Ward
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440007', '750e8400-e29b-41d4-a716-446655440006', '101241', 6.4281, 3.4219, true),
    ('850e8400-e29b-41d4-a716-446655440008', '750e8400-e29b-41d4-a716-446655440006', '101242', 6.4282, 3.4220, true),
    ('850e8400-e29b-41d4-a716-446655440009', '750e8400-e29b-41d4-a716-446655440006', '101243', 6.4283, 3.4221, true);

-- Sample Postal Codes for Ikoyi Ward
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440010', '750e8400-e29b-41d4-a716-446655440007', '101233', 6.4520, 3.4350, true),
    ('850e8400-e29b-41d4-a716-446655440011', '750e8400-e29b-41d4-a716-446655440007', '101234', 6.4521, 3.4351, true),
    ('850e8400-e29b-41d4-a716-446655440012', '750e8400-e29b-41d4-a716-446655440007', '101235', 6.4522, 3.4352, true);

-- Sample Postal Codes for Asokoro Ward (Abuja)
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440013', '750e8400-e29b-41d4-a716-446655440011', '900001', 9.0765, 7.3986, true),
    ('850e8400-e29b-41d4-a716-446655440014', '750e8400-e29b-41d4-a716-446655440011', '900002', 9.0766, 7.3987, true),
    ('850e8400-e29b-41d4-a716-446655440015', '750e8400-e29b-41d4-a716-446655440011', '900003', 9.0767, 7.3988, true);

-- Sample Postal Codes for Maitama Ward (Abuja)
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440016', '750e8400-e29b-41d4-a716-446655440012', '904001', 9.0765, 7.3986, true),
    ('850e8400-e29b-41d4-a716-446655440017', '750e8400-e29b-41d4-a716-446655440012', '904002', 9.0766, 7.3987, true),
    ('850e8400-e29b-41d4-a716-446655440018', '750e8400-e29b-41d4-a716-446655440012', '904003', 9.0767, 7.3988, true);

-- Sample Postal Codes for Kano Municipal
INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440019', '750e8400-e29b-41d4-a716-446655440011', '700001', 12.0022, 8.5920, true),
    ('850e8400-e29b-41d4-a716-446655440020', '750e8400-e29b-41d4-a716-446655440011', '700002', 12.0023, 8.5921, true),
    ('850e8400-e29b-41d4-a716-446655440021', '750e8400-e29b-41d4-a716-446655440011', '700003', 12.0024, 8.5922, true);

-- Add some rural postal codes (without coordinates)
INSERT INTO postal_codes (id, ward_id, postal_code, urban) VALUES
    ('850e8400-e29b-41d4-a716-446655440022', '750e8400-e29b-41d4-a716-446655440003', '100301', false),
    ('850e8400-e29b-41d4-a716-446655440023', '750e8400-e29b-41d4-a716-446655440003', '100302', false),
    ('850e8400-e29b-41d4-a716-446655440024', '750e8400-e29b-41d4-a716-446655440004', '100401', false),
    ('850e8400-e29b-41d4-a716-446655440025', '750e8400-e29b-41d4-a716-446655440004', '100402', false);
