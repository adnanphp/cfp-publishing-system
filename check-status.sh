#!/bin/bash
echo "📊 CFP System Status"
echo "==================="

echo ""
echo "1. Docker Services:"
docker ps --filter "name=cfp"

echo ""
echo "2. Backend (http://localhost:3000/health):"
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/health && echo " - OK" || echo " - DOWN"

echo ""
echo "3. Frontend (http://localhost:5173):"
curl -s -o /dev/null -w "%{http_code}" http://localhost:5173 && echo " - OK" || echo " - DOWN"

echo ""
echo "4. ML Service (http://localhost:8000):"
curl -s -o /dev/null -w "%{http_code}" http://localhost:8000 && echo " - OK" || echo " - DOWN"

echo ""
echo "5. Process IDs:"
if [ -f ".pids" ]; then
    cat .pids
else
    echo "No .pids file found"
fi
