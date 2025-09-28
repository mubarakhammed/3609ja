#!/usr/bin/env python3
"""
Generate Sample Nigerian Ward Data

Since comprehensive ward data is not readily available online, this script generates
sample wards based on typical Nigerian administrative structures:

- Each LGA typically has 8-15 wards
- Ward names usually follow patterns like:
  * Geographic references (North, South, East, West, Central)
  * Local place names
  * Numbered wards (Ward I, Ward II, etc.)
  * Community names

This script creates a realistic sample dataset that can be used for development
and testing purposes.
"""

import uuid
import random

# Sample ward naming patterns commonly used in Nigeria
WARD_PATTERNS = {
    "directional": ["North", "South", "East", "West", "Central", "Northeast", "Northwest", "Southeast", "Southwest"],
    "numbered_roman": ["I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"],
    "numbered_ordinal": ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th"],
    "generic_descriptors": ["Urban", "Rural", "Market", "River", "Hill", "Town", "Village", "Commercial", "Residential"],
    "common_nigerian_names": [
        "Ahmadu", "Aliyu", "Bello", "Danjuma", "Garba", "Hassan", "Ibrahim", "Jibril", "Musa", "Sani",
        "Adamu", "Abubakar", "Mamman", "Shehu", "Umar", "Yakubu", "Ismail", "Lawal", "Mohammed", "Nuhu",
        "Adebayo", "Adewale", "Afolabi", "Akeem", "Babatunde", "Dele", "Emeka", "Femi", "Gbenga", "Kemi",
        "Adeola", "Adunni", "Bukola", "Damilola", "Folake", "Kehinde", "Modupe", "Ronke", "Seun", "Yemi"
    ]
}

def generate_uuid():
    """Generate a UUID string"""
    return str(uuid.uuid4())

def generate_ward_name(lga_name, ward_index, total_wards):
    """
    Generate a realistic ward name based on LGA name and patterns
    """
    # Choose naming strategy based on random selection
    strategy = random.choice([
        "directional_simple",
        "numbered_roman", 
        "numbered_ordinal",
        "lga_plus_direction",
        "descriptive",
        "local_name"
    ])
    
    if strategy == "directional_simple":
        return f"{random.choice(WARD_PATTERNS['directional'])} Ward"
    
    elif strategy == "numbered_roman":
        return f"Ward {WARD_PATTERNS['numbered_roman'][ward_index % len(WARD_PATTERNS['numbered_roman'])]}"
    
    elif strategy == "numbered_ordinal":
        return f"{WARD_PATTERNS['numbered_ordinal'][ward_index % len(WARD_PATTERNS['numbered_ordinal'])]} Ward"
    
    elif strategy == "lga_plus_direction":
        direction = random.choice(WARD_PATTERNS['directional'])
        return f"{lga_name} {direction}"
    
    elif strategy == "descriptive":
        descriptor = random.choice(WARD_PATTERNS['generic_descriptors'])
        return f"{descriptor} Ward"
    
    elif strategy == "local_name":
        name = random.choice(WARD_PATTERNS['common_nigerian_names'])
        return f"{name} Ward"
    
    # Fallback
    return f"Ward {ward_index + 1}"

def generate_ward_code(ward_index):
    """Generate a ward code in format WARD-XXXX"""
    return f"WARD-{ward_index + 1:04d}"

def get_typical_ward_count(lga_name):
    """
    Determine realistic number of wards for an LGA
    Based on typical Nigerian administrative structures
    """
    # Urban/major LGAs tend to have more wards
    urban_indicators = ["Municipal", "Metropolitan", "City", "Urban", "Capital", "Central"]
    is_urban = any(indicator in lga_name for indicator in urban_indicators)
    
    if is_urban:
        return random.randint(10, 15)  # Urban LGAs: 10-15 wards
    else:
        return random.randint(8, 12)   # Rural LGAs: 8-12 wards

def generate_sample_wards_sql():
    """
    Generate sample ward data for all LGAs in the database
    This creates a realistic dataset for development/testing
    """
    print("Generating sample Nigerian ward data...")
    print("Note: This is sample data for development purposes")
    print("Real ward data would need to be sourced from INEC or local government records")
    
    sql_lines = ["-- Sample Nigerian Ward Data (for development/testing)"]
    sql_lines.append("-- Note: Replace with authentic data from official sources")
    sql_lines.append("")
    sql_lines.append("INSERT INTO wards (id, lga_id, name, code) VALUES")
    
    # Sample LGA data (we'll need to query the real LGA table for production)
    # For now, creating sample entries
    sample_lgas = [
        "Abuja Municipal",
        "Ikeja", 
        "Victoria Island",
        "Surulere",
        "Kano Municipal",
        "Kaduna North",
        "Port Harcourt",
        "Ibadan North",
        "Jos North",
        "Maiduguri"
    ]
    
    ward_entries = []
    ward_counter = 1
    
    for lga_index, lga_name in enumerate(sample_lgas):
        ward_count = get_typical_ward_count(lga_name)
        
        for ward_index in range(ward_count):
            ward_id = generate_uuid()
            ward_name = generate_ward_name(lga_name, ward_index, ward_count)
            ward_code = generate_ward_code(ward_counter - 1)
            
            # Escape single quotes in ward names
            escaped_ward_name = ward_name.replace("'", "''")
            
            # For sample data, using placeholder LGA ID
            lga_placeholder = f"(SELECT id FROM lgas WHERE name LIKE '%{lga_name}%' LIMIT 1)"
            
            ward_entries.append(
                f"    ('{ward_id}', {lga_placeholder}, '{escaped_ward_name}', '{ward_code}')"
            )
            ward_counter += 1
    
    sql_lines.append(",\n".join(ward_entries) + ";")
    
    # Add comment about total wards
    sql_lines.append("")
    sql_lines.append(f"-- Sample dataset contains {ward_counter - 1} wards")
    sql_lines.append("-- Nigeria has approximately 8,809 wards nationwide")
    sql_lines.append("-- This is a representative sample for development purposes")
    
    return "\n".join(sql_lines)

def main():
    print("🏛️  Nigeria Geo API - Sample Ward Data Generator")
    print("=" * 60)
    
    # Generate sample ward SQL
    wards_sql = generate_sample_wards_sql()
    
    # Write to file
    filename = 'seed_sample_wards.sql'
    with open(filename, 'w') as f:
        f.write(wards_sql)
    
    print(f"✅ Generated sample ward data!")
    print(f"📁 File created: {filename}")
    print(f"⚠️  IMPORTANT: This is sample data for development")
    print(f"   Replace with authentic ward data from official sources")
    print(f"   Sources: INEC, Local Government Records, NPC")
    
    # Show sample of generated ward names
    print(f"\n📊 Sample ward names generated:")
    lines = wards_sql.split('\n')
    ward_lines = [line for line in lines if line.strip().startswith("('") and "Ward" in line]
    for i, line in enumerate(ward_lines[:10]):  # Show first 10
        # Extract ward name from SQL
        parts = line.split("'")
        if len(parts) >= 6:
            ward_name = parts[5]  # Ward name is the 3rd quoted string
            print(f"   • {ward_name}")
    
    if len(ward_lines) > 10:
        print(f"   ... and {len(ward_lines) - 10} more")

if __name__ == "__main__":
    main()