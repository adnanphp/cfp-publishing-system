#!/bin/bash
# shutdown.sh
echo "Stopping services..."
pkill -f cfp-backend || true
pkill -f vite || true
pkill -f uvicorn || true
cd ~/projects/cfp
docker-compose down
echo "✅ Stopped"
