#!/bin/bash
echo "🚀 Starting CFP System (Fixed Version)..."

# Kill existing processes
pkill -f "cfp-backend" 2>/dev/null || true
pkill -f "vite" 2>/dev/null || true
pkill -f "uvicorn src.main:app" 2>/dev/null || true
sleep 2

# Start Docker
echo "1. Starting Docker services..."
docker-compose up -d
sleep 3

# Start Minimal Backend (from backend directory)
echo "2. Starting Minimal Backend..."
cd backend
cargo build --release
cd ..
./backend/target/release/cfp-backend > backend.log 2>&1 &
BACKEND_PID=$!
echo "   Minimal Backend PID: $BACKEND_PID"
sleep 3

# Start Frontend (from frontend directory)
echo "3. Starting Frontend..."
cd frontend
npm run dev > ../frontend.log 2>&1 &
FRONTEND_PID=$!
echo "   Frontend PID: $FRONTEND_PID"
cd ..
sleep 3

# Start ML Service (from ml-service directory)
echo "4. Starting ML Service..."
cd ml-service
source venv/bin/activate
uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload > ../ml-service.log 2>&1 &
ML_PID=$!
echo "   ML Service PID: $ML_PID"
cd ..

# Save PIDs
cat > .pids << EOPIDS
BACKEND_PID=$BACKEND_PID
FRONTEND_PID=$FRONTEND_PID
ML_PID=$ML_PID
EOPIDS

echo ""
echo "✅ All services started successfully!"
echo ""
echo "Access URLs:"
echo "  Frontend Dashboard: http://localhost:5173/working-dashboard.html"
echo "  Backend API:        http://localhost:3000/"
echo "  ML Service:         http://localhost:8000/"
echo ""
echo "To stop: ./stop-all.sh"
