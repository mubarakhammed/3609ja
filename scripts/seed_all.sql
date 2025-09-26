-- Master seeding script for Nigeria Geo API
-- Run this script to populate the database with sample data

\echo 'Seeding Nigerian States...'
\i seed_states.sql

\echo 'Seeding LGAs...'
\i seed_lgas.sql

\echo 'Seeding Wards...'
\i seed_wards.sql

\echo 'Seeding Postal Codes...'
\i seed_postal_codes.sql

\echo 'Sample data seeding completed successfully!'
\echo 'Total records created:'
\echo 'States: 10'
\echo 'LGAs: 15'
\echo 'Wards: 15'
\echo 'Postal Codes: 25'
