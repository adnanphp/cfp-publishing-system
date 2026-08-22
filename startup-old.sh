#!/bin/bash
# startup.sh - Complete CFP Project Startup

echo "🚀 Starting CFP Publishing System..."

# 1. Start Docker services
echo "Starting Docker services..."
cd ~/projects/cfp
docker-compose up -d

# 2. Wait for services to be ready
echo "Waiting for services to be ready..."
sleep 5

# 3. Run database migrations
echo "Running database migrations..."
cd backend
sqlx database create
sqlx migrate run

# 4. Build and start backend
echo "Building backend..."
cargo build --release
cp target/release/cfp-backend ~/projects/cfp/

# 5. Start backend
echo "Starting backend..."
cd ~/projects/cfp
nohup ./cfp-backend > backend.log 2>&1 &
BACKEND_PID=$!

# 6. Build and start frontend
echo "Building frontend..."
cd frontend
pnpm install
pnpm run build

# 7. Start frontend
echo "Starting frontend..."
nohup pnpm run preview > frontend.log 2>&1 &
FRONTEND_PID=$!

# 8. Start ML service
echo "Starting ML service..."
cd ../ml-service
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
nohup uvicorn src.main:app --host 0.0.0.0 --port 8000 > ml-service.log 2>&1 &
ML_PID=$!

# 9. Create systemd service files
echo "Creating systemd services..."
sudo tee /etc/systemd/system/cfp-backend.service << EOF
[Unit]
Description=CFP Backend Service
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=/home/$USER/projects/cfp
ExecStart=/home/$USER/projects/cfp/cfp-backend
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable cfp-backend
sudo systemctl start cfp-backend

# 10. Display status
echo ""
echo "✅ CFP System Started Successfully!"
echo ""
echo "Services:"
echo "  PostgreSQL: localhost:5432"
echo "  Redis:      localhost:6380"
echo "  Backend:    localhost:3000"
echo "  Frontend:   localhost:5173"
echo "  ML Service: localhost:8000"
echo ""
echo "Access:"
echo "  Web Interface: http://localhost:5173"
echo "  API:          http://localhost:3000/api"
echo "  API Docs:     http://localhost:3000/api/docs"
echo ""
echo "Logs:"
echo "  Backend:   tail -f ~/projects/cfp/backend.log"
echo "  Frontend:  tail -f ~/projects/cfp/frontend.log"
echo "  ML:        tail -f ~/projects/cfp/ml-service.log"
echo ""
echo "To stop all services:"
echo "  ./shutdown.sh"
