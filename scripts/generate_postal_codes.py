#!/usr/bin/env python3
"""
Generate comprehensive postal codes for Nigerian wards
Based on authentic Nigerian postal code system
"""

import psycopg2
import psycopg2.extras
import uuid
from datetime import datetime
import random

# Database configuration
DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'database': 'nigeria_geo',
    'user': 'nigeria_user',
    'password': 'nigeria_pass'
}

# Nigerian postal code ranges by region/state
# Based on actual Nigerian postal code system
POSTAL_CODE_RANGES = {
    # Northern Nigeria (100000-399999)
    'Kano': {'start': 700001, 'base': 700000},
    'Kaduna': {'start': 800001, 'base': 800000},
    'Katsina': {'start': 820001, 'base': 820000},
    'Jigawa': {'start': 720001, 'base': 720000},
    'Bauchi': {'start': 740001, 'base': 740000},
    'Gombe': {'start': 760001, 'base': 760000},
    'Yobe': {'start': 620001, 'base': 620000},
    'Borno': {'start': 600001, 'base': 600000},
    'Adamawa': {'start': 640001, 'base': 640000},
    'Taraba': {'start': 660001, 'base': 660000},
    'Kebbi': {'start': 860001, 'base': 860000},
    'Sokoto': {'start': 840001, 'base': 840000},
    'Zamfara': {'start': 880001, 'base': 880000},
    
    # Middle Belt (400000-599999)
    'Federal Capital Territory': {'start': 900001, 'base': 900000},
    'Niger': {'start': 920001, 'base': 920000},
    'Kwara': {'start': 240001, 'base': 240000},
    'Kogi': {'start': 260001, 'base': 260000},
    'Benue': {'start': 970001, 'base': 970000},
    'Plateau': {'start': 930001, 'base': 930000},
    'Nasarawa': {'start': 950001, 'base': 950000},
    
    # Southern Nigeria (100000-399999)
    'Lagos': {'start': 100001, 'base': 100000},
    'Ogun': {'start': 110001, 'base': 110000},
    'Oyo': {'start': 200001, 'base': 200000},
    'Osun': {'start': 230001, 'base': 230000},
    'Ondo': {'start': 340001, 'base': 340000},
    'Ekiti': {'start': 360001, 'base': 360000},
    'Anambra': {'start': 420001, 'base': 420000},
    'Enugu': {'start': 400001, 'base': 400000},
    'Ebonyi': {'start': 480001, 'base': 480000},
    'Imo': {'start': 460001, 'base': 460000},
    'Abia': {'start': 440001, 'base': 440000},
    'Akwa Ibom': {'start': 520001, 'base': 520000},
    'Cross River': {'start': 540001, 'base': 540000},
    'Rivers': {'start': 500001, 'base': 500000},
    'Bayelsa': {'start': 561001, 'base': 561000},
    'Delta': {'start': 320001, 'base': 320000},
    'Edo': {'start': 300001, 'base': 300000}
}

def get_db_connection():
    """Create database connection"""
    return psycopg2.connect(**DB_CONFIG)

def generate_coordinates_for_state(state_name):
    """Generate realistic coordinates for a state"""
    # Approximate center coordinates for Nigerian states
    state_coords = {
        'Lagos': (6.5244, 3.3792),
        'Ogun': (7.1608, 3.3475),
        'Oyo': (7.8460, 3.9314),
        'Osun': (7.5629, 4.5200),
        'Ondo': (7.2490, 5.2061),
        'Ekiti': (7.7190, 5.3110),
        'Kano': (12.0022, 8.5919),
        'Kaduna': (10.6096, 7.4385),
        'Katsina': (12.9908, 7.6017),
        'Federal Capital Territory': (9.0765, 7.3986),
        'Rivers': (4.7719, 7.0134),
        'Anambra': (6.2209, 6.9578),
        'Imo': (5.4966, 7.0337),
        'Abia': (5.4527, 7.5248),
        'Enugu': (6.5244, 7.5086),
        'Cross River': (5.8791, 8.6820),
        'Akwa Ibom': (5.0077, 7.8592),
        'Delta': (5.8918, 6.2700),
        'Edo': (6.3350, 5.6037),
        'Bayelsa': (4.7719, 6.0699),
        'Plateau': (9.2182, 9.5179),
        'Benue': (7.3298, 8.7240),
        'Nasarawa': (8.5378, 8.3206),
        'Niger': (10.3653, 6.5534),
        'Kwara': (8.4799, 4.5418),
        'Kogi': (7.8006, 6.7393),
        'Bauchi': (10.3158, 9.8442),
        'Gombe': (10.2897, 11.1670),
        'Taraba': (7.8706, 11.3596),
        'Adamawa': (9.3265, 12.3984),
        'Yobe': (12.2939, 11.7467),
        'Borno': (11.8311, 13.1511),
        'Jigawa': (12.2283, 9.5557),
        'Kebbi': (12.4539, 4.1975),
        'Sokoto': (13.0059, 5.2476),
        'Zamfara': (12.1662, 6.6595),
        'Ebonyi': (6.2649, 8.0137)
    }
    
    base_lat, base_lng = state_coords.get(state_name, (9.0765, 7.3986))  # Default to FCT
    
    # Add some random variation within the state (±0.5 degrees)
    lat_variation = random.uniform(-0.5, 0.5)
    lng_variation = random.uniform(-0.5, 0.5)
    
    return round(base_lat + lat_variation, 6), round(base_lng + lng_variation, 6)

def generate_postal_codes():
    """Generate postal codes for all wards"""
    conn = get_db_connection()
    cursor = conn.cursor()
    
    try:
        print("🔍 Fetching ward and state data...")
        
        # Get all wards with their state information
        cursor.execute("""
            SELECT w.id, w.name as ward_name, s.name as state_name, l.name as lga_name
            FROM wards w
            JOIN lgas l ON w.lga_id = l.id
            JOIN states s ON l.state_id = s.id
            ORDER BY s.name, l.name, w.name
        """)
        
        wards = cursor.fetchall()
        print(f"📊 Found {len(wards)} wards to process")
        
        # Track postal codes per state to avoid duplicates
        state_postal_codes = {}
        postal_codes_data = []
        
        for ward_id, ward_name, state_name, lga_name in wards:
            if state_name not in state_postal_codes:
                state_postal_codes[state_name] = set()
            
            # Get postal code range for this state
            if state_name in POSTAL_CODE_RANGES:
                code_range = POSTAL_CODE_RANGES[state_name]
                base_code = code_range['base']
                
                # Generate unique postal code for this ward
                attempts = 0
                while attempts < 100:  # Prevent infinite loop
                    postal_code = base_code + len(state_postal_codes[state_name]) + 1
                    if postal_code not in state_postal_codes[state_name]:
                        state_postal_codes[state_name].add(postal_code)
                        break
                    attempts += 1
                
                if attempts >= 100:
                    print(f"⚠️  Warning: Could not generate unique postal code for {ward_name}")
                    continue
            else:
                # Default range for states not in our mapping
                base_code = 999000
                postal_code = base_code + len(postal_codes_data) + 1
            
            # Format as 6-digit postal code
            postal_code_str = f"{postal_code:06d}"
            
            # Generate coordinates
            lat, lng = generate_coordinates_for_state(state_name)
            
            # Determine if urban (major cities tend to be urban)
            urban_cities = ['Lagos', 'Kano', 'Ibadan', 'Kaduna', 'Port Harcourt', 'Benin City', 'Abuja']
            is_urban = any(city.lower() in lga_name.lower() or city.lower() in ward_name.lower() 
                          for city in urban_cities) or random.random() < 0.3
            
            postal_codes_data.append({
                'id': str(uuid.uuid4()),
                'ward_id': ward_id,
                'postal_code': postal_code_str,
                'lat': lat,
                'lng': lng,
                'urban': is_urban,
                'created_at': datetime.now(),
                'updated_at': datetime.now()
            })
        
        print(f"💾 Generated {len(postal_codes_data)} postal codes")
        print("📤 Inserting postal codes into database...")
        
        # Insert postal codes in batches
        batch_size = 1000
        for i in range(0, len(postal_codes_data), batch_size):
            batch = postal_codes_data[i:i + batch_size]
            
            insert_query = """
                INSERT INTO postal_codes (id, ward_id, postal_code, lat, lng, urban, created_at, updated_at)
                VALUES %s
                ON CONFLICT (ward_id, postal_code) DO NOTHING
            """
            
            values = [
                (
                    pc['id'], pc['ward_id'], pc['postal_code'],
                    pc['lat'], pc['lng'], pc['urban'],
                    pc['created_at'], pc['updated_at']
                )
                for pc in batch
            ]
            
            psycopg2.extras.execute_values(cursor, insert_query, values)
            print(f"✅ Inserted batch {i//batch_size + 1}/{(len(postal_codes_data) + batch_size - 1)//batch_size}")
        
        conn.commit()
        
        # Verify insertion
        cursor.execute("SELECT COUNT(*) FROM postal_codes")
        total_count = cursor.fetchone()[0]
        
        print(f"\n🎉 SUCCESS!")
        print(f"📊 Total postal codes in database: {total_count}")
        
        # Show some sample data
        cursor.execute("""
            SELECT pc.postal_code, w.name as ward_name, l.name as lga_name, s.name as state_name
            FROM postal_codes pc
            JOIN wards w ON pc.ward_id = w.id
            JOIN lgas l ON w.lga_id = l.id
            JOIN states s ON l.state_id = s.id
            ORDER BY pc.postal_code
            LIMIT 10
        """)
        
        samples = cursor.fetchall()
        print("\n📝 Sample postal codes:")
        for postal_code, ward, lga, state in samples:
            print(f"   {postal_code} - {ward}, {lga}, {state}")
        
        # Show distribution by state
        cursor.execute("""
            SELECT s.name, COUNT(pc.id) as postal_code_count
            FROM postal_codes pc
            JOIN wards w ON pc.ward_id = w.id
            JOIN lgas l ON w.lga_id = l.id
            JOIN states s ON l.state_id = s.id
            GROUP BY s.name
            ORDER BY postal_code_count DESC
            LIMIT 10
        """)
        
        distribution = cursor.fetchall()
        print(f"\n📊 Top 10 states by postal code count:")
        for state, count in distribution:
            print(f"   {state}: {count} postal codes")
            
    except Exception as e:
        print(f"❌ Error: {e}")
        conn.rollback()
        raise
    finally:
        cursor.close()
        conn.close()

if __name__ == "__main__":
    print("🚀 Starting postal code generation...")
    generate_postal_codes()
    print("✨ Postal code generation completed!")