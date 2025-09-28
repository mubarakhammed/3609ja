#!/bin/bash

# Complete Server Diagnostic Script
# Run this ON YOUR SERVER (20.63.52.179)

echo "🔍 Nigeria Geo API - Complete Server Diagnostics"
echo "================================================"
echo "Run this script ON your server (20.63.52.179)"
echo ""

# 1. Check if app is running and what it's binding to
echo "1. APPLICATION STATUS"
echo "===================="
echo "Checking if nigeria-geo-api is running..."
if pgrep -f "nigeria-geo-api" > /dev/null; then
    echo "✅ Application is running"
    echo "PIDs: $(pgrep -f 'nigeria-geo-api')"
    echo ""
    echo "Process details:"
    ps aux | grep nigeria-geo-api | grep -v grep
else
    echo "❌ Application is NOT running"
    echo "You need to start it: ./target/release/nigeria-geo-api"
fi

echo ""
echo "2. PORT BINDING CHECK"
echo "===================="
echo "Checking what's listening on port 3000..."

# Check with netstat first, then ss as fallback
if command -v netstat >/dev/null 2>&1; then
    netstat_output=$(netstat -tlnp 2>/dev/null | grep :3000)
elif command -v ss >/dev/null 2>&1; then
    netstat_output=$(ss -tlnp | grep :3000 2>/dev/null)
else
    netstat_output=""
fi

if [ -n "$netstat_output" ]; then
    echo "✅ Port 3000 is in use:"
    echo "$netstat_output"
    echo ""
    
    # Critical check: Is it binding to 0.0.0.0 or 127.0.0.1?
    if echo "$netstat_output" | grep -q "0.0.0.0:3000"; then
        echo "✅ GOOD: Server is listening on ALL interfaces (0.0.0.0:3000)"
        echo "   This means it should accept external connections."
    elif echo "$netstat_output" | grep -q "127.0.0.1:3000"; then
        echo "❌ PROBLEM: Server is only listening on localhost (127.0.0.1:3000)"
        echo "   This is why external access fails!"
        echo "   Fix: Ensure SERVER_HOST=0.0.0.0 in your .env file and restart the app"
    else
        echo "⚠️  Server is listening on some other interface:"
        echo "$netstat_output"
    fi
else
    echo "❌ Nothing is listening on port 3000"
    echo "   The application is not running or not binding to port 3000"
fi

echo ""
echo "3. FIREWALL CHECK"
echo "================="

# Check UFW (Ubuntu Firewall)
if command -v ufw >/dev/null 2>&1; then
    echo "UFW Status:"
    ufw_status=$(sudo ufw status 2>/dev/null)
    echo "$ufw_status"
    
    if echo "$ufw_status" | grep -q "Status: active"; then
        echo ""
        if echo "$ufw_status" | grep -q "3000"; then
            echo "✅ Port 3000 is allowed in UFW"
        else
            echo "❌ Port 3000 is NOT allowed in UFW"
            echo "   Fix: sudo ufw allow 3000"
        fi
    else
        echo "✅ UFW is inactive (not blocking)"
    fi
else
    echo "UFW not found"
fi

# Check iptables
echo ""
echo "Checking iptables rules..."
if command -v iptables >/dev/null 2>&1; then
    # Check if there are any DROP rules for port 3000
    drop_rules=$(sudo iptables -L | grep -i drop | grep -i 3000 2>/dev/null || echo "")
    if [ -n "$drop_rules" ]; then
        echo "❌ Found iptables DROP rules that might block port 3000:"
        echo "$drop_rules"
    else
        echo "✅ No obvious iptables DROP rules for port 3000"
    fi
else
    echo "iptables not found"
fi

echo ""
echo "4. CLOUD PROVIDER SECURITY GROUPS"
echo "=================================="
echo "⚠️  IMPORTANT: Check your cloud provider settings!"
echo ""
echo "If you're using:"
echo "• AWS: Check Security Groups - ensure port 3000 is open (0.0.0.0/0)"
echo "• Azure: Check Network Security Groups - ensure port 3000 is allowed"
echo "• Google Cloud: Check Firewall Rules - ensure port 3000 is open"
echo "• DigitalOcean: Check Firewall settings in the control panel"
echo ""

echo ""
echo "5. LOCAL CONNECTION TEST"
echo "========================"
echo "Testing local connection (from the server itself)..."

if command -v curl >/dev/null 2>&1; then
    # Test localhost
    local_response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/api/v1/health 2>/dev/null || echo "failed")
    if [ "$local_response" = "200" ]; then
        echo "✅ Local connection works (http://localhost:3000)"
    else
        echo "❌ Local connection failed (response: $local_response)"
        echo "   This suggests the application has issues"
    fi
    
    # Test 127.0.0.1
    local_ip_response=$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/api/v1/health 2>/dev/null || echo "failed")
    if [ "$local_ip_response" = "200" ]; then
        echo "✅ Local IP connection works (http://127.0.0.1:3000)"
    else
        echo "❌ Local IP connection failed (response: $local_ip_response)"
    fi
    
    # Test the actual server IP
    server_ip_response=$(curl -s -o /dev/null -w "%{http_code}" http://20.63.52.179:3000/api/v1/health 2>/dev/null || echo "failed")
    if [ "$server_ip_response" = "200" ]; then
        echo "✅ Server IP connection works (http://20.63.52.179:3000)"
        echo "   If this works but external access doesn't, it's a cloud firewall issue"
    else
        echo "❌ Server IP connection failed (response: $server_ip_response)"
    fi
else
    echo "curl not available - install with: sudo apt install curl"
fi

echo ""
echo "6. ENVIRONMENT CHECK"
echo "==================="
echo "Current directory: $(pwd)"
echo "SERVER_HOST env var: ${SERVER_HOST:-'Not set'}"
echo "SERVER_PORT env var: ${SERVER_PORT:-'Not set'}"

if [ -f ".env" ]; then
    echo ""
    echo "✅ .env file found:"
    echo "SERVER_HOST: $(grep SERVER_HOST .env 2>/dev/null || echo 'Not found in .env')"
    echo "SERVER_PORT: $(grep SERVER_PORT .env 2>/dev/null || echo 'Not found in .env')"
else
    echo "❌ .env file not found in current directory"
    echo "   Make sure you're running this from the app directory"
fi

echo ""
echo "7. SYSTEM INFO"
echo "=============="
echo "OS: $(cat /etc/os-release 2>/dev/null | grep PRETTY_NAME | cut -d'=' -f2 | tr -d '\"' || uname -s)"
echo "Server IP addresses:"
ip addr show 2>/dev/null | grep -E "inet.*global" | awk '{print "  " $2}' || ifconfig 2>/dev/null | grep -E "inet.*broadcast" | awk '{print "  " $2}'

echo ""
echo "🔧 NEXT STEPS"
echo "=============="
echo ""

if ! pgrep -f "nigeria-geo-api" > /dev/null; then
    echo "1. ❌ START THE APPLICATION:"
    echo "   cd /path/to/your/app"
    echo "   ./target/release/nigeria-geo-api"
    echo ""
fi

if netstat -tlnp 2>/dev/null | grep :3000 | grep -q "127.0.0.1:3000"; then
    echo "2. ❌ FIX BINDING ISSUE:"
    echo "   Edit .env file: SERVER_HOST=0.0.0.0"
    echo "   Restart the application"
    echo ""
fi

if command -v ufw >/dev/null 2>&1 && sudo ufw status 2>/dev/null | grep -q "Status: active" && ! sudo ufw status 2>/dev/null | grep -q "3000"; then
    echo "3. ❌ OPEN FIREWALL PORT:"
    echo "   sudo ufw allow 3000"
    echo ""
fi

echo "4. ⚠️  CHECK CLOUD PROVIDER FIREWALL:"
echo "   Most likely cause: Cloud provider security groups blocking port 3000"
echo "   - AWS: Security Groups"
echo "   - Azure: Network Security Groups" 
echo "   - Google Cloud: VPC Firewall Rules"
echo "   - DigitalOcean: Cloud Firewall"
echo ""

echo "5. 🧪 TEST AGAIN:"
echo "   From external: curl http://20.63.52.179:3000/api/v1/health"
echo ""

echo "🏁 Diagnostic complete!"
echo "If local connections work but external don't, it's definitely a firewall issue."