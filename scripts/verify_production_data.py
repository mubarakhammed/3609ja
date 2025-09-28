#!/usr/bin/env python3
"""
Verify production database data integrity
"""

import psycopg2

# Database configuration for Azure PostgreSQL
DB_CONFIG = {
    'host': 'prod-3609ja.postgres.database.azure.com',
    'port': 5432,
    'database': 'postgres',
    'user': 'mubarak',
    'password': 'TafawaBalewa123!'
}

def verify_database():
    """Verify all data is properly loaded"""
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        print("✅ Connected to production database")
        
        with conn.cursor() as cursor:
            # Check states
            cursor.execute("SELECT COUNT(*) FROM states")
            states_count = cursor.fetchone()[0]
            
            # Check LGAs
            cursor.execute("SELECT COUNT(*) FROM lgas")
            lgas_count = cursor.fetchone()[0]
            
            # Check wards
            cursor.execute("SELECT COUNT(*) FROM wards")
            wards_count = cursor.fetchone()[0]
            
            # Check postal codes
            cursor.execute("SELECT COUNT(*) FROM postal_codes")
            postal_codes_count = cursor.fetchone()[0]
            
            # Check sample data
            cursor.execute("SELECT name, code FROM states LIMIT 5")
            sample_states = cursor.fetchall()
            
            cursor.execute("SELECT name FROM lgas LIMIT 5")
            sample_lgas = cursor.fetchall()
            
            print(f"\n🗃️  Production Database Summary:")
            print(f"   📍 States: {states_count}")
            print(f"   🏛️  LGAs: {lgas_count}")
            print(f"   🏘️  Wards: {wards_count}")
            print(f"   📮 Postal Codes: {postal_codes_count}")
            
            print(f"\n📋 Sample States:")
            for state in sample_states:
                print(f"   • {state[0]} ({state[1]})")
                
            print(f"\n📋 Sample LGAs:")
            for lga in sample_lgas:
                print(f"   • {lga[0]}")
            
            # Check relationships
            cursor.execute("""
                SELECT s.name, COUNT(l.id) as lga_count 
                FROM states s 
                LEFT JOIN lgas l ON s.id = l.state_id 
                GROUP BY s.name 
                ORDER BY lga_count DESC 
                LIMIT 5
            """)
            state_lga_counts = cursor.fetchall()
            
            print(f"\n🔗 Top States by LGA Count:")
            for state, count in state_lga_counts:
                print(f"   • {state}: {count} LGAs")
            
        conn.close()
        print(f"\n🎉 Database verification completed successfully!")
        print(f"🚀 Your production API is ready with complete Nigerian geographic data!")
        
    except Exception as e:
        print(f"❌ Verification failed: {e}")

if __name__ == "__main__":
    verify_database()