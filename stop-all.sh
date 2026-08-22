#!/bin/bash
echo "🛑 Stopping CFP System..."

echo "Stopping backend..."
pkill -f "cfp-backend" || true

echo "Stopping frontend..."
pkill -f "vite" || true

echo "Stopping ML service..."
pkill -f "uvicorn src.main:app" || true

echo "Stopping Docker..."
docker-compose down

rm -f .pids
echo ""
echo "✅ All services stopped."
