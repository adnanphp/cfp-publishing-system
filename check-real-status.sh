#!/bin/bash
echo "🔍 Checking CFP Services (Detailed)..."
echo "====================================="

echo ""
echo "1. Process Status:"
echo "------------------"
if [ -f ".pids" ]; then
    source .pids
    echo "Backend PID: $BACKEND_PID - $(ps -p $BACKEND_PID > /dev/null && echo "✅ Running" || echo "❌ Stopped")"
    echo "Frontend PID: $FRONTEND_PID - $(ps -p $FRONTEND_PID > /dev/null && echo "✅ Running" || echo "❌ Stopped")"
    echo "ML Service PID: $ML_PID - $(ps -p $ML_PID > /dev/null && echo "✅ Running" || echo "❌ Stopped")"
else
    echo "No .pids file found"
fi

echo ""
echo "2. Port Status:"
echo "---------------"
echo "Port 3000 (Backend): $(netstat -tuln | grep :3000 > /dev/null && echo "✅ Listening" || echo "❌ Not listening")"
echo "Port 5173 (Frontend): $(netstat -tuln | grep :5173 > /dev/null && echo "✅ Listening" || echo "❌ Not listening")"
echo "Port 8000 (ML): $(netstat -tuln | grep :8000 > /dev/null && echo "✅ Listening" || echo "❌ Not listening")"
echo "Port 6380 (Redis): $(netstat -tuln | grep :6380 > /dev/null && echo "✅ Listening" || echo "❌ Not listening")"

echo ""
echo "3. Service Tests:"
echo "-----------------"
echo "Backend (port 3000):"
timeout 2 curl -s -o /dev/null -w "  HTTP Status: %{http_code}\n" http://localhost:3000/ || echo "  ❌ Not responding"

echo ""
echo "ML Service (port 8000):"
timeout 2 curl -s -o /dev/null -w "  HTTP Status: %{http_code}\n" http://localhost:8000/ || echo "  ❌ Not responding"
timeout 2 curl -s http://localhost:8000/ && echo "  ✅ Returns JSON"

echo ""
echo "Frontend (port 5173):"
timeout 2 curl -s -o /dev/null -w "  HTTP Status: %{http_code}\n" http://localhost:5173/ || echo "  ❌ Not responding"

echo ""
echo "4. Docker:"
echo "----------"
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
