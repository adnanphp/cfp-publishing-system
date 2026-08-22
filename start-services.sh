#!/bin/bash

echo "🚀 Starting CFP Services..."

# Kill existing services
echo "Stopping existing services..."
pkill -f cfp-backend || true
pkill -f vite || true
pkill -f uvicorn || true
sleep 2

# Start Docker
echo "Starting Docker..."
docker-compose up -d
sleep 3

# Start Backend
echo "Starting Backend..."
cd backend
cargo run --release &
cd ..
echo "Backend started"
sleep 3

# Start Frontend (in development mode)
echo "Starting Frontend..."
cd frontend
npm run dev &
cd ..
echo "Frontend started"
sleep 3

# Start ML Service
echo "Starting ML Service..."
cd ml-service
source venv/bin/activate
uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload &
cd ..
echo "ML Service started"

echo ""
echo "✅ All services started!"
echo ""
echo "Access URLs:"
echo "Frontend: http://localhost:5173"
echo "Backend:  http://localhost:3000"
echo "ML:       http://localhost:8000"
echo ""
echo "Press Ctrl+C to stop all services"
