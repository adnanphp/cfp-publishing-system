#!/bin/bash
echo "🛑 Stopping CFP System..."
echo ""
echo "Stopping backend..."
pkill -f "cfp-backend" || echo "Backend not running"
echo "Stopping frontend..."
pkill -f "vite" || echo "Frontend not running"
echo "Stopping ML service..."
pkill -f "uvicorn src.main:app" || echo "ML service not running"
echo "Stopping Docker..."
docker-compose down
echo ""
echo "✅ All services stopped."
