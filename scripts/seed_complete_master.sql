-- ==============================================
-- COMPLETE NIGERIAN GEOGRAPHIC DATA
-- All 36 States + FCT with Major LGAs and Sample Data
-- ==============================================

\echo '🌍 Starting complete Nigerian geographic data seeding...'

\echo '📊 Seeding all 37 states (36 states + FCT)...'
\i seed_complete_all.sql

\echo '🏘️ Seeding sample wards and postal codes...'
\i seed_complete_wards_postal.sql

\echo '✅ Complete Nigerian geographic data seeding finished!'
\echo ''
\echo '📈 SUMMARY:'
\echo '   States: 37 (36 states + FCT)'
\echo '   LGAs: 143+ (major LGAs from all states)'
\echo '   Wards: 40+ (sample wards from major LGAs)'
\echo '   Postal Codes: 48+ (with real coordinates for major cities)'
\echo ''
\echo '🎯 Coverage includes:'
\echo '   • All 6 geopolitical zones'
\echo '   • Major cities: Lagos, Abuja, Kano, Port Harcourt, Ibadan, etc.'
\echo '   • Real coordinates for urban areas'
\echo '   • Rural postal codes without coordinates'
\echo '   • Complete state-LGA-ward-postal hierarchy'
\echo ''
\echo '🚀 Your Nigeria Geo API is now ready with comprehensive data!'
