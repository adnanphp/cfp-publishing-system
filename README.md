
#  CFP Publishing System

A complete publishing platform for charitable foundations with backend API, frontend dashboard, and ML-powered services.

## ✨ Features

- **Backend API** - Rust/Actix-web RESTful API with PostgreSQL
- **Frontend Dashboard** - SvelteKit admin interface
- **ML Service** - FastAPI microservice for text analysis
- **Docker Support** - Easy deployment with Redis caching
- **Real-time Updates** - WebSocket notifications

## 🏗 Architecture

```
CFP System
├── backend/        # Rust API (port 3000)
├── frontend/       # SvelteKit UI (port 5173)
├── ml-service/     # Python ML API (port 8000)
└── docker-compose  # PostgreSQL + Redis
```

##  Quick Start

### Prerequisites
- Rust 1.70+
- Node.js 18+ / pnpm
- Python 3.10+
- Docker & Docker Compose
- PostgreSQL 14+ (or use Docker)

### 1. Clone & Setup
```bash
git clone https://github.com/adnanphp/cfp-publishing-system.git
cd cfp-publishing-system
```

### 2. Start All Services

**One-command startup:**
```bash
chmod +x cfp.sh
./cfp.sh start
```

**Or manually:**
```bash
# Start Docker (PostgreSQL + Redis)
docker-compose up -d

# Build & start backend
cd backend && cargo build --release && cd ..
./backend/target/release/cfp-backend > backend.log 2>&1 &

# Start frontend
cd frontend && pnpm install && pnpm run dev > ../frontend.log 2>&1 &

# Start ML service
cd ml-service
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
uvicorn src.main:app --host 0.0.0.0 --port 8000 > ../ml-service.log 2>&1 &
```

### 3. Access the System

| Service | URL |
|---------|-----|
|  Frontend Dashboard | http://localhost:5173 |
|  Backend API | http://localhost:3000 |
|  API Docs | http://localhost:3000/api/docs |
|  ML Service | http://localhost:8000 |
|  ML Docs | http://localhost:8000/docs |

### 4. Management Commands

```bash
./cfp.sh start          # Start all services
./cfp.sh stop           # Stop all services
./cfp.sh status         # Check service status
./cfp.sh restart        # Restart all services
./cfp.sh logs [service] # View logs (backend|frontend|ml)
./cfp.sh test           # Test all endpoints
```

## 📡 API Endpoints

### Health & Status
- `GET /health` - System health check
- `GET /status` - System status

### Core Resources
- `GET /api/members` - List members
- `GET /api/texts` - List texts
- `GET /api/authors` - List authors
- `GET /api/downloads` - Download statistics
- `GET /api/downloads/stats` - Download analytics

### Example Response
```json
{
  "service": "cfp-backend",
  "status": "healthy",
  "timestamp": "2026-08-22T22:09:57.115Z"
}
```

## 🔧 Development

### Backend Development
```bash
cd backend
cargo run
# Watches for changes with cargo-watch
cargo install cargo-watch
cargo watch -x run
```

### Frontend Development
```bash
cd frontend
pnpm install
pnpm run dev
# Visit http://localhost:5173
```

### ML Service Development
```bash
cd ml-service
source venv/bin/activate
uvicorn src.main:app --reload
```

## 🐳 Docker Deployment

```bash
# Build and start all services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Docker Compose Configuration
```yaml
services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_DB: cfp_db
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    ports:
      - "5432:5432"

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
```

## 🧪 Testing

```bash
# Test all backend endpoints
./cfp.sh test

# Run Rust tests
cd backend && cargo test

# Test ML service
curl http://localhost:8000/health
```

## 📊 Project Structure

```
cfp-publishing-system/
├── backend/
│   ├── src/
│   │   ├── api/          # HTTP handlers & routes
│   │   ├── domain/       # Business logic
│   │   └── main.rs       # Entry point
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   └── routes/       # SvelteKit pages
│   └── package.json
├── ml-service/
│   ├── src/
│   │   └── main.py       # FastAPI app
│   └── requirements.txt
├── docker-compose.yml
├── cfp.sh                # Management script
└── README.md
```

## 🛠️ Technology Stack

| Component | Technology |
|-----------|------------|
| Backend | Rust + Actix-web |
| Frontend | SvelteKit + Vite |
| ML Service | Python + FastAPI |
| Database | PostgreSQL |
| Cache | Redis |
| Container | Docker + Compose |

## 📝 Environment Variables

Create `.env` file:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/cfp_db
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

## 📄 License

MIT License - see [LICENSE](LICENSE) file

##  Acknowledgments

- Built with ❤️ using Rust, SvelteKit, and FastAPI
- Thanks to all contributors and open-source libraries

---
