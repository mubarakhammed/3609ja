-- Comprehensive Nigerian States, LGAs, Wards, and Postal Codes
-- This is a sample dataset for demonstration purposes

-- Insert all 36 states + FCT
INSERT INTO states (name, code) VALUES
('Abia', 'NG-AB'),
('Adamawa', 'NG-AD'),
('Akwa Ibom', 'NG-AK'),
('Anambra', 'NG-AN'),
('Bauchi', 'NG-BA'),
('Bayelsa', 'NG-BY'),
('Benue', 'NG-BE'),
('Borno', 'NG-BO'),
('Cross River', 'NG-CR'),
('Delta', 'NG-DE'),
('Ebonyi', 'NG-EB'),
('Edo', 'NG-ED'),
('Ekiti', 'NG-EK'),
('Enugu', 'NG-EN'),
('Gombe', 'NG-GO'),
('Imo', 'NG-IM'),
('Jigawa', 'NG-JI'),
('Kaduna', 'NG-KD'),
('Kano', 'NG-KN'),
('Katsina', 'NG-KT'),
('Kebbi', 'NG-KE'),
('Kogi', 'NG-KO'),
('Kwara', 'NG-KW'),
('Lagos', 'NG-LA'),
('Nasarawa', 'NG-NA'),
('Niger', 'NG-NI'),
('Ogun', 'NG-OG'),
('Ondo', 'NG-ON'),
('Osun', 'NG-OS'),
('Oyo', 'NG-OY'),
('Plateau', 'NG-PL'),
('Rivers', 'NG-RI'),
('Sokoto', 'NG-SO'),
('Taraba', 'NG-TA'),
('Yobe', 'NG-YO'),
('Zamfara', 'NG-ZA'),
('Abuja', 'NG-FC')
ON CONFLICT (code) DO NOTHING;

-- Insert major LGAs for Lagos State
INSERT INTO lgas (state_id, name, code) 
SELECT s.id, lga_name, lga_code
FROM states s,
(VALUES 
    ('Agege', 'NG-LA-01'),
    ('Ajeromi-Ifelodun', 'NG-LA-02'),
    ('Alimosho', 'NG-LA-03'),
    ('Amuwo-Odofin', 'NG-LA-04'),
    ('Apapa', 'NG-LA-05'),
    ('Badagry', 'NG-LA-06'),
    ('Epe', 'NG-LA-07'),
    ('Eti-Osa', 'NG-LA-08'),
    ('Ibeju-Lekki', 'NG-LA-09'),
    ('Ifako-Ijaiye', 'NG-LA-10'),
    ('Ikeja', 'NG-LA-11'),
    ('Ikorodu', 'NG-LA-12'),
    ('Kosofe', 'NG-LA-13'),
    ('Lagos Island', 'NG-LA-14'),
    ('Lagos Mainland', 'NG-LA-15'),
    ('Mushin', 'NG-LA-16'),
    ('Ojo', 'NG-LA-17'),
    ('Oshodi-Isolo', 'NG-LA-18'),
    ('Shomolu', 'NG-LA-19'),
    ('Surulere', 'NG-LA-20')
) AS lgas(lga_name, lga_code)
WHERE s.code = 'NG-LA'
ON CONFLICT (code) DO NOTHING;

-- Insert major LGAs for Abuja (FCT)
INSERT INTO lgas (state_id, name, code) 
SELECT s.id, lga_name, lga_code
FROM states s,
(VALUES 
    ('Abuja Municipal', 'NG-FC-01'),
    ('Bwari', 'NG-FC-02'),
    ('Gwagwalada', 'NG-FC-03'),
    ('Kuje', 'NG-FC-04'),
    ('Kwali', 'NG-FC-05'),
    ('Abaji', 'NG-FC-06')
) AS lgas(lga_name, lga_code)
WHERE s.code = 'NG-FC'
ON CONFLICT (code) DO NOTHING;

-- Insert major LGAs for Kano State
INSERT INTO lgas (state_id, name, code) 
SELECT s.id, lga_name, lga_code
FROM states s,
(VALUES 
    ('Dala', 'NG-KN-01'),
    ('Fagge', 'NG-KN-02'),
    ('Gwale', 'NG-KN-03'),
    ('Kano Municipal', 'NG-KN-04'),
    ('Kumbotso', 'NG-KN-05'),
    ('Nassarawa', 'NG-KN-06'),
    ('Tarauni', 'NG-KN-07'),
    ('Ungogo', 'NG-KN-08')
) AS lgas(lga_name, lga_code)
WHERE s.code = 'NG-KN'
ON CONFLICT (code) DO NOTHING;

-- Insert sample wards for Ikeja LGA
INSERT INTO wards (lga_id, name, code)
SELECT l.id, ward_name, ward_code
FROM lgas l,
(VALUES 
    ('Ikeja Central', 'NG-LA-11-01'),
    ('Ikeja North', 'NG-LA-11-02'),
    ('Ikeja South', 'NG-LA-11-03'),
    ('Ojodu', 'NG-LA-11-04'),
    ('Oregun', 'NG-LA-11-05'),
    ('Alausa', 'NG-LA-11-06')
) AS wards(ward_name, ward_code)
WHERE l.code = 'NG-LA-11'
ON CONFLICT (code) DO NOTHING;

-- Insert sample wards for Victoria Island (Eti-Osa)
INSERT INTO wards (lga_id, name, code)
SELECT l.id, ward_name, ward_code
FROM lgas l,
(VALUES 
    ('Victoria Island I', 'NG-LA-08-01'),
    ('Victoria Island II', 'NG-LA-08-02'),
    ('Ikoyi I', 'NG-LA-08-03'),
    ('Ikoyi II', 'NG-LA-08-04'),
    ('Lekki I', 'NG-LA-08-05'),
    ('Lekki II', 'NG-LA-08-06')
) AS wards(ward_name, ward_code)
WHERE l.code = 'NG-LA-08'
ON CONFLICT (code) DO NOTHING;

-- Insert sample wards for Abuja Municipal
INSERT INTO wards (lga_id, name, code)
SELECT l.id, ward_name, ward_code
FROM lgas l,
(VALUES 
    ('Asokoro', 'NG-FC-01-01'),
    ('Garki', 'NG-FC-01-02'),
    ('Maitama', 'NG-FC-01-03'),
    ('Utako', 'NG-FC-01-04'),
    ('Wuse I', 'NG-FC-01-05'),
    ('Wuse II', 'NG-FC-01-06')
) AS wards(ward_name, ward_code)
WHERE l.code = 'NG-FC-01'
ON CONFLICT (code) DO NOTHING;

-- Insert sample postal codes for Ikeja Central
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('100001', 6.4474, 3.3903, true),
    ('100002', 6.4480, 3.3910, true),
    ('100003', 6.4490, 3.3920, true),
    ('100004', 6.4500, 3.3930, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-LA-11-01'
ON CONFLICT DO NOTHING;

-- Insert sample postal codes for Victoria Island
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('101241', 6.4281, 3.4219, true),
    ('101242', 6.4290, 3.4220, true),
    ('101243', 6.4300, 3.4230, true),
    ('101244', 6.4310, 3.4240, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-LA-08-01'
ON CONFLICT DO NOTHING;

-- Insert sample postal codes for Abuja (Asokoro)
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('900001', 9.0765, 7.3986, true),
    ('900002', 9.0770, 7.3990, true),
    ('900003', 9.0780, 7.4000, true),
    ('900004', 9.0790, 7.4010, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-FC-01-01'
ON CONFLICT DO NOTHING;

-- Insert sample postal codes for Kano Municipal
INSERT INTO postal_codes (ward_id, postal_code, lat, lng, urban)
SELECT w.id, postal_code, lat, lng, urban
FROM wards w,
(VALUES 
    ('700001', 12.0022, 8.5920, true),
    ('700002', 12.0030, 8.5930, true),
    ('700003', 12.0040, 8.5940, true),
    ('700004', 12.0050, 8.5950, true)
) AS postal_codes(postal_code, lat, lng, urban)
WHERE w.code = 'NG-KN-04-01'
ON CONFLICT DO NOTHING;
