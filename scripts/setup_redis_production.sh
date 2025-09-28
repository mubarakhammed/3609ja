#!/bin/bash

# Redis Setup Script for Ubuntu/Debian Production Server
# Run this on your production server (20.63.52.179)

echo "🚀 Setting up Redis for Nigeria Geo API Production Server"

# Update system packages
echo "📦 Updating system packages..."
sudo apt update

# Install Redis
echo "🔧 Installing Redis Server..."
sudo apt install -y redis-server

# Configure Redis for production
echo "⚙️  Configuring Redis for production..."

# Backup original config
sudo cp /etc/redis/redis.conf /etc/redis/redis.conf.backup

# Configure Redis to bind to all interfaces (for external access)
sudo sed -i 's/bind 127.0.0.1 ::1/bind 0.0.0.0/' /etc/redis/redis.conf

# Set protected mode to no (allow external connections)
sudo sed -i 's/protected-mode yes/protected-mode no/' /etc/redis/redis.conf

# Set maxmemory policy for production
echo "maxmemory 256mb" | sudo tee -a /etc/redis/redis.conf
echo "maxmemory-policy allkeys-lru" | sudo tee -a /etc/redis/redis.conf

# Enable persistence
echo "save 900 1" | sudo tee -a /etc/redis/redis.conf
echo "save 300 10" | sudo tee -a /etc/redis/redis.conf
echo "save 60 10000" | sudo tee -a /etc/redis/redis.conf

# Set log level
echo "loglevel notice" | sudo tee -a /etc/redis/redis.conf

# Configure Redis to start on boot
echo "🔄 Enabling Redis to start on boot..."
sudo systemctl enable redis-server

# Start Redis service
echo "▶️  Starting Redis service..."
sudo systemctl start redis-server

# Check Redis status
echo "✅ Checking Redis status..."
sudo systemctl status redis-server

# Test Redis connection
echo "🧪 Testing Redis connection..."
redis-cli ping

# Open firewall for Redis (if ufw is enabled)
echo "🔥 Configuring firewall for Redis..."
if sudo ufw --version >/dev/null 2>&1; then
    echo "UFW detected, opening port 6379..."
    sudo ufw allow 6379/tcp
else
    echo "UFW not detected, please manually open port 6379 if using another firewall"
fi

# Show Redis info
echo "📊 Redis Information:"
redis-cli info server | head -10

echo ""
echo "🎉 Redis setup completed!"
echo ""
echo "Redis is now accessible at:"
echo "  - Local: redis://127.0.0.1:6379"  
echo "  - External: redis://20.63.52.179:6379"
echo ""
echo "⚠️  SECURITY NOTE: Redis is configured to allow external connections"
echo "   For production, consider setting up authentication:"
echo "   1. Add 'requirepass your-strong-password' to /etc/redis/redis.conf"
echo "   2. Restart Redis: sudo systemctl restart redis-server"
echo "   3. Update REDIS_URL to: redis://:password@20.63.52.179:6379"
echo ""
echo "🔧 Useful Redis commands:"
echo "  - Check status: sudo systemctl status redis-server"
echo "  - Restart: sudo systemctl restart redis-server"
echo "  - View logs: sudo journalctl -u redis-server -f"
echo "  - Connect: redis-cli -h 20.63.52.179"