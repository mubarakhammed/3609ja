#!/usr/bin/env python3
"""
Populate production database with all Nigerian geographic data
"""

import psycopg2
import os
import sys

# Database configuration for Azure PostgreSQL
DB_CONFIG = {
    'host': 'prod-3609ja.postgres.database.azure.com',
    'port': 5432,
    'database': 'postgres',
    'user': 'mubarak',
    'password': 'TafawaBalewa123!'
}

def execute_sql_file(conn, file_path):
    """Execute SQL file"""
    print(f"📄 Executing {file_path}...")
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            sql_content = file.read()
            
        with conn.cursor() as cursor:
            cursor.execute(sql_content)
            conn.commit()
            print(f"✅ Successfully executed {file_path}")
            return True
    except Exception as e:
        print(f"❌ Error executing {file_path}: {e}")
        conn.rollback()
        return False

def get_db_connection():
    """Create database connection"""
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        print("✅ Connected to Azure PostgreSQL database")
        return conn
    except Exception as e:
        print(f"❌ Failed to connect to database: {e}")
        return None

def check_data_counts(conn):
    """Check how many records were inserted"""
    try:
        with conn.cursor() as cursor:
            cursor.execute("SELECT COUNT(*) FROM states")
            states_count = cursor.fetchone()[0]
            
            cursor.execute("SELECT COUNT(*) FROM lgas")
            lgas_count = cursor.fetchone()[0]
            
            cursor.execute("SELECT COUNT(*) FROM wards")
            wards_count = cursor.fetchone()[0]
            
            print(f"\n📊 Data Summary:")
            print(f"   States: {states_count}")
            print(f"   LGAs: {lgas_count}")
            print(f"   Wards: {wards_count}")
            
    except Exception as e:
        print(f"❌ Error checking data counts: {e}")

def main():
    """Main function to populate the database"""
    print("🚀 Starting production database population...")
    
    # Get database connection
    conn = get_db_connection()
    if not conn:
        sys.exit(1)
    
    try:
        # List of SQL files to execute in order
        sql_files = [
            'scripts/seed_states_all37.sql',
            'scripts/seed_lgas_774_authentic.sql',
            'scripts/seed_comprehensive_wards.sql'
        ]
        
        # Execute each SQL file
        for sql_file in sql_files:
            if os.path.exists(sql_file):
                success = execute_sql_file(conn, sql_file)
                if not success:
                    print(f"❌ Failed to execute {sql_file}, stopping...")
                    break
            else:
                print(f"⚠️  File not found: {sql_file}")
        
        # Check final data counts
        check_data_counts(conn)
        
        print("\n🎉 Database population completed!")
        
    except Exception as e:
        print(f"❌ Unexpected error: {e}")
    finally:
        conn.close()
        print("🔐 Database connection closed")

if __name__ == "__main__":
    main()