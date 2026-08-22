#!/bin/bash
echo "🚀 Starting CFP System (Simple Version)..."
echo ""

# Get the project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_DIR"

echo "1. Stopping any existing services..."
pkill -f "cfp-backend" 2>/dev/null || echo "No backend running"
pkill -f "vite" 2>/dev/null || echo "No frontend running"
pkill -f "uvicorn src.main:app" 2>/dev/null || echo "No ML service running"
sleep 2

echo ""
echo "2. Starting Docker (Redis)..."
docker-compose up -d
sleep 3

echo ""
echo "3. Starting Backend..."
if [ -f "backend/target/release/cfp-backend" ]; then
    echo "   Using existing backend binary"
else
    echo "   Building backend..."
    cd backend
    cargo build --release
    cd ..
fi
./backend/target/release/cfp-backend > backend.log 2>&1 &
echo "   Backend PID: $!"
sleep 3

echo ""
echo "4. Starting Frontend..."
cd frontend
npm run dev > ../frontend.log 2>&1 &
echo "   Frontend PID: $!"
cd ..
sleep 3

echo ""
echo "5. Starting ML Service..."
cd ml-service
if [ -d "venv" ]; then
    source venv/bin/activate
    uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload > ../ml-service.log 2>&1 &
    echo "   ML Service PID: $!"
else
    echo "   ⚠️  No venv found, skipping ML service"
fi
cd ..

echo ""
echo "✅ Startup complete!"
echo ""
echo "Quick test:"
echo "  curl http://localhost:3000/health"
echo "  curl http://localhost:8000/"
echo ""
echo "Open dashboard:"
echo "  http://localhost:5173/working-dashboard.html"
echo ""
echo "Press Ctrl+C to stop this script (services will continue running)"
echo "Use './simple-stop.sh' to stop all services"

# Keep script running
while true; do
    sleep 3600
done
