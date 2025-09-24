-- Insert sample Nigerian states
INSERT INTO states (name, code) VALUES
('Lagos', 'NG-LA'),
('Abuja', 'NG-FC'),
('Kano', 'NG-KN'),
('Rivers', 'NG-RI'),
('Kaduna', 'NG-KD')
ON CONFLICT (code) DO NOTHING;

-- Insert sample LGAs for Lagos
INSERT INTO lgas (state_id, name, code) 
SELECT s.id, lga_name, lga_code
FROM states s,
(VALUES 
    ('Ikeja', 'NG-LA-01'),
    ('Eti-Osa', 'NG-LA-02'),
    ('Lagos Island', 'NG-LA-03'),
    ('Lagos Mainland', 'NG-LA-04'),
    ('Surulere', 'NG-LA-05')
) AS lgas(lga_name, lga_code)
WHERE s.code = 'NG-LA'
ON CONFLICT (code) DO NOTHING;

-- Insert sample LGAs for Abuja
INSERT INTO lgas (state_id, name, code) 
SELECT s.id, lga_name, lga_code
FROM states s,
(VALUES 
    ('Abuja Municipal', 'NG-FC-01'),
    ('Bwari', 'NG-FC-02'),
    ('Gwagwalada', 'NG-FC-03'),
    ('Kuje', 'NG-FC-04'),
    ('Kwali', 'NG-FC-05')
) AS lgas(lga_name, lga_code)
WHERE s.code = 'NG-FC'
ON CONFLICT (code) DO NOTHING;

-- Insert sample wards for Ikeja LGA
INSERT INTO wards (lga_id, name, code)
SELECT l.id, ward_name, ward_code
FROM lgas l,
(VALUES 
    ('Ikeja Central', 'NG-LA-01-01'),
    ('Ikeja North', 'NG-LA-01-02'),
    ('Ikeja South', 'NG-LA-01-03'),
    ('Ojodu', 'NG-LA-01-04'),
    ('Oregun', 'NG-LA-01-05')
) AS wards(ward_name, ward_code)
WHERE l.code = 'NG-LA-01'
ON CONFLICT (code) DO NOTHING;

-- Insert sample wards for Eti-Osa LGA
INSERT INTO wards (lga_id, name, code)
SELECT l.id, ward_name, ward_code
FROM lgas l,
(VALUES 
    ('Victoria Island', 'NG-LA-02-01'),
    ('Ikoyi', 'NG-LA-02-02'),
    ('Lekki', 'NG-LA-02-03'),
    ('Ajah', 'NG-LA-02-04'),
    ('Eti-Osa East', 'NG-LA-02-05')
) AS wards(ward_name, ward_code)
WHERE l.code = 'NG-LA-02'
ON CONFLICT (code) DO NOTHING;

-- Insert sample postal codes for Ikeja Central ward
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('100001', 6.4474, 3.3903, true),
    ('100002', 6.4480, 3.3910, true),
    ('100003', 6.4490, 3.3920, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-LA-01-01'
ON CONFLICT DO NOTHING;

-- Insert sample postal codes for Victoria Island ward
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('101241', 6.4281, 3.4219, true),
    ('101242', 6.4290, 3.4220, true),
    ('101243', 6.4300, 3.4230, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-LA-02-01'
ON CONFLICT DO NOTHING;

-- Insert sample postal codes for Abuja Municipal ward
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('900001', 9.0765, 7.3986, true),
    ('900002', 9.0770, 7.3990, true),
    ('900003', 9.0780, 7.4000, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-FC-01-01'
ON CONFLICT DO NOTHING;
