#!/bin/bash

# Nigeria Geo API Server Manager
# Simple script to start, stop, and check your Rust API server

PID_FILE="/tmp/nigeria-geo-api.pid"
LOG_FILE="server.log"
APP_NAME="nigeria-geo-api"

start() {
    if [ -f "$PID_FILE" ] && ps -p $(cat "$PID_FILE") > /dev/null 2>&1; then
        echo "✅ Server is already running (PID: $(cat $PID_FILE))"
        return 0
    fi
    
    echo "🚀 Starting Nigeria Geo API server..."
    
    # Build if needed
    if [ ! -f "target/release/$APP_NAME" ]; then
        echo "📦 Building application..."
        cargo build --release
    fi
    
    # Start server in background
    nohup ./target/release/$APP_NAME > $LOG_FILE 2>&1 &
    echo $! > "$PID_FILE"
    
    sleep 2
    
    if ps -p $(cat "$PID_FILE") > /dev/null 2>&1; then
        echo "✅ Server started successfully!"
        echo "📋 PID: $(cat $PID_FILE)"
        echo "📄 Logs: $LOG_FILE"
        echo "🌐 API: http://localhost:3000"
    else
        echo "❌ Failed to start server"
        rm -f "$PID_FILE"
        exit 1
    fi
}

stop() {
    if [ ! -f "$PID_FILE" ]; then
        echo "⚠️  Server is not running"
        return 0
    fi
    
    PID=$(cat "$PID_FILE")
    
    if ! ps -p "$PID" > /dev/null 2>&1; then
        echo "⚠️  Server is not running"
        rm -f "$PID_FILE"
        return 0
    fi
    
    echo "🛑 Stopping server (PID: $PID)..."
    kill -TERM "$PID"
    sleep 3
    
    # Force kill if still running
    if ps -p "$PID" > /dev/null 2>&1; then
        echo "💀 Force killing server..."
        kill -KILL "$PID"
    fi
    
    rm -f "$PID_FILE"
    echo "✅ Server stopped"
}

status() {
    if [ -f "$PID_FILE" ] && ps -p $(cat "$PID_FILE") > /dev/null 2>&1; then
        PID=$(cat "$PID_FILE")
        echo "✅ Server is running"
        echo "📋 PID: $PID"
        echo "🧠 Memory: $(ps -p $PID -o rss= | awk '{print $1/1024 " MB"}')"
        echo "⏱️  Uptime: $(ps -p $PID -o etime= | awk '{print $1}')"
        echo "🌐 API: http://localhost:3000"
        echo "📄 Logs: $LOG_FILE"
    else
        echo "❌ Server is not running"
        [ -f "$PID_FILE" ] && rm -f "$PID_FILE"
    fi
}

logs() {
    if [ -f "$LOG_FILE" ]; then
        echo "📄 Server logs (last 20 lines):"
        echo "================================"
        tail -20 "$LOG_FILE"
        echo "================================"
        echo "💡 Use 'tail -f $LOG_FILE' to follow logs in real-time"
    else
        echo "❌ No log file found"
    fi
}

case "${1:-}" in
    start)
        start
        ;;
    stop)
        stop
        ;;
    restart)
        stop
        sleep 2
        start
        ;;
    status)
        status
        ;;
    logs)
        logs
        ;;
    *)
        echo "🇳🇬 Nigeria Geo API Server Manager"
        echo ""
        echo "Usage: $0 {start|stop|restart|status|logs}"
        echo ""
        echo "Commands:"
        echo "  start   - Start the server in background"
        echo "  stop    - Stop the server"
        echo "  restart - Restart the server" 
        echo "  status  - Show server status"
        echo "  logs    - Show recent server logs"
        echo ""
        echo "Examples:"
        echo "  $0 start     # Start the server"
        echo "  $0 status    # Check if running"
        echo "  $0 logs      # View logs"
        echo "  $0 stop      # Stop the server"
        ;;
esac