#!/bin/bash

# Nigeria Geo API Production Deployment Script
# This script builds and deploys the API without Docker

set -e

echo "🚀 Starting Nigeria Geo API Production Deployment..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed. Please install Rust first."
    exit 1
fi

# Check if PostgreSQL client is available
if ! command -v psql &> /dev/null; then
    echo "⚠️  PostgreSQL client not found. Database operations may fail."
fi

# Load environment variables
if [ -f .env ]; then
    echo "📋 Loading environment variables..."
    export $(cat .env | grep -v '^#' | xargs)
else
    echo "❌ .env file not found. Please create one from env.example"
    exit 1
fi

# Build the application in release mode
echo "🔨 Building application in release mode..."
cargo build --release

# Run database migrations (if psql is available)
if command -v psql &> /dev/null; then
    echo "📊 Running database migrations..."
    for migration in migrations/*.sql; do
        if [ -f "$migration" ]; then
            echo "Running $migration..."
            psql "$DATABASE_URL" -f "$migration" || echo "⚠️  Migration $migration may have already been applied"
        fi
    done
else
    echo "⚠️  Skipping database migrations - psql not available"
fi

# Create systemd service file (optional)
create_systemd_service() {
    echo "📝 Creating systemd service file..."
    sudo tee /etc/systemd/system/nigeria-geo-api.service > /dev/null <<EOF
[Unit]
Description=Nigeria Geo API
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=nigeria-geo
WorkingDirectory=$(pwd)
Environment=RUST_LOG=$RUST_LOG
Environment=DATABASE_URL=$DATABASE_URL
Environment=REDIS_URL=$REDIS_URL
Environment=SERVER_HOST=$SERVER_HOST
Environment=SERVER_PORT=$SERVER_PORT
ExecStart=$(pwd)/target/release/nigeria-geo-api
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

    echo "✅ Systemd service file created at /etc/systemd/system/nigeria-geo-api.service"
    echo "   Run the following commands to enable and start the service:"
    echo "   sudo systemctl daemon-reload"
    echo "   sudo systemctl enable nigeria-geo-api"
    echo "   sudo systemctl start nigeria-geo-api"
}

# Check if running as root or with sudo for systemd service creation
if [ "$EUID" -eq 0 ]; then
    read -p "🤔 Create systemd service? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        create_systemd_service
    fi
fi

echo "✅ Build completed successfully!"
echo "🎯 Binary location: $(pwd)/target/release/nigeria-geo-api"
echo ""
echo "🚀 To start the server manually:"
echo "   ./target/release/nigeria-geo-api"
echo ""
echo "📊 API will be available at: http://$SERVER_HOST:$SERVER_PORT"
echo "🏥 Health check: http://$SERVER_HOST:$SERVER_PORT/health"
echo ""
echo "🔍 To check logs when running as systemd service:"
echo "   sudo journalctl -u nigeria-geo-api -f"