#!/bin/bash

echo "🔍 Nigeria Geo API Server Troubleshooting Script"
echo "================================================"

# Check if the server is running on the expected port
echo "1. Checking if server is listening on port 3000..."
if netstat -tlnp 2>/dev/null | grep -q ":3000"; then
    echo "✅ Server is listening on port 3000"
    netstat -tlnp | grep ":3000"
else
    echo "❌ Server is NOT listening on port 3000"
fi

echo ""
echo "2. Checking server process..."
if pgrep -f "nigeria-geo-api" > /dev/null; then
    echo "✅ Nigeria Geo API process is running"
    ps aux | grep "nigeria-geo-api" | grep -v grep
else
    echo "❌ Nigeria Geo API process is NOT running"
fi

echo ""
echo "3. Testing local connectivity..."
if curl -s -o /dev/null -w "%{http_code}" "http://localhost:3000/api/v1/health" | grep -q "200"; then
    echo "✅ Server responds locally on localhost:3000"
else
    echo "❌ Server does NOT respond locally"
    echo "Trying to connect..."
    curl -v "http://localhost:3000/api/v1/health" 2>&1 || echo "Connection failed"
fi

echo ""
echo "4. Testing internal network connectivity..."
INTERNAL_IP=$(ip route get 8.8.8.8 | awk '{print $7; exit}' 2>/dev/null || hostname -I | awk '{print $1}')
echo "Internal IP: $INTERNAL_IP"

if curl -s -o /dev/null -w "%{http_code}" "http://$INTERNAL_IP:3000/api/v1/health" | grep -q "200"; then
    echo "✅ Server responds on internal IP: $INTERNAL_IP"
else
    echo "❌ Server does NOT respond on internal IP"
fi

echo ""
echo "5. Checking firewall status..."
if command -v ufw &> /dev/null; then
    echo "UFW Firewall status:"
    sudo ufw status
    echo ""
    echo "Checking if port 3000 is allowed:"
    sudo ufw status | grep 3000 || echo "Port 3000 is NOT explicitly allowed"
elif command -v firewall-cmd &> /dev/null; then
    echo "Firewalld status:"
    sudo firewall-cmd --state
    echo "Checking if port 3000 is open:"
    sudo firewall-cmd --list-ports | grep 3000 || echo "Port 3000 is NOT open"
else
    echo "No common firewall detected (ufw/firewalld)"
fi

echo ""
echo "6. Checking iptables rules..."
if command -v iptables &> /dev/null; then
    echo "Iptables INPUT rules for port 3000:"
    sudo iptables -L INPUT -n | grep 3000 || echo "No specific iptables rules for port 3000"
fi

echo ""
echo "7. Testing external connectivity (from outside)..."
echo "Try this command from your local machine:"
echo "curl -v http://20.63.52.179:3000/api/v1/health"

echo ""
echo "🔧 COMMON FIXES:"
echo "=================="
echo "1. Open firewall port:"
echo "   sudo ufw allow 3000/tcp"
echo "   # OR for firewalld:"
echo "   sudo firewall-cmd --permanent --add-port=3000/tcp"
echo "   sudo firewall-cmd --reload"
echo ""
echo "2. Check cloud provider security groups (Azure/AWS/GCP)"
echo "   - Allow inbound traffic on port 3000"
echo "   - Source: 0.0.0.0/0 (or your specific IPs)"
echo ""
echo "3. Restart the application with explicit binding:"
echo "   SERVER_HOST=0.0.0.0 SERVER_PORT=3000 ./target/release/nigeria-geo-api"
echo ""
echo "4. Check if another service is using port 3000:"
echo "   sudo netstat -tlnp | grep 3000"
echo "   sudo lsof -i :3000"