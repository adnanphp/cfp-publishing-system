# CFP Publishing System

> A full-stack publishing platform for charitable foundations, combining a Rust backend, SvelteKit dashboard, PostgreSQL database, Redis caching, WebSocket notifications, and an ML-powered text analysis service.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![SvelteKit](https://img.shields.io/badge/SvelteKit-Frontend-orange?logo=svelte)](https://kit.svelte.dev/)
[![Python](https://img.shields.io/badge/Python-3.10%2B-blue?logo=python)](https://www.python.org/)
[![FastAPI](https://img.shields.io/badge/FastAPI-ML%20Service-009688?logo=fastapi)](https://fastapi.tiangolo.com/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-336791?logo=postgresql)](https://www.postgresql.org/)
[![Redis](https://img.shields.io/badge/Redis-Cache-DC382D?logo=redis)](https://redis.io/)
[![Docker](https://img.shields.io/badge/Docker-Compose-2496ED?logo=docker)](https://www.docker.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 🚀 Overview

**CFP Publishing System** is an end-to-end publishing platform designed for charitable foundations and content-driven organizations.

The system separates core application logic, user interface, and machine-learning functionality into independent services:

* **Rust + Actix-web** — high-performance backend API
* **SvelteKit** — responsive administrative dashboard
* **FastAPI + Python** — ML-powered text analysis service
* **PostgreSQL** — persistent relational data storage
* **Redis** — caching and fast-access data
* **WebSockets** — real-time notifications
* **Docker Compose** — local development and service orchestration

The architecture is designed to demonstrate modern **full-stack, backend, API, ML-service, and containerized application development**.

---

## ✨ Key Features

### Backend API

* RESTful API built with Rust and Actix-web
* PostgreSQL integration
* Member, author, text, and download resources
* Health and system-status endpoints
* API documentation

### Frontend Dashboard

* SvelteKit-based administration interface
* Vite development environment
* Integration with backend REST APIs
* Real-time system updates

### ML Service

* Python-based FastAPI microservice
* Text analysis functionality
* Independent deployment from the main backend
* Interactive API documentation

### Infrastructure

* PostgreSQL database
* Redis caching
* Docker Compose orchestration
* Centralized service management through `cfp.sh`

### Real-Time Communication

* WebSocket-based notifications
* Event-driven communication between services and clients

---

## 🏗️ System Architecture

```text
                         ┌──────────────────────┐
                         │   SvelteKit Frontend │
                         │      Port 5173       │
                         └──────────┬───────────┘
                                    │
                              REST / WebSocket
                                    │
                                    ▼
                         ┌──────────────────────┐
                         │   Rust / Actix-web   │
                         │      Port 3000       │
                         └───────┬────────┬─────┘
                                 │        │
                       ┌─────────┘        └──────────┐
                       ▼                             ▼
              ┌─────────────────┐          ┌─────────────────┐
              │   PostgreSQL     │          │      Redis      │
              │     Database     │          │     Cache       │
              │     Port 5432    │          │     Port 6379   │
              └─────────────────┘          └─────────────────┘

                                    │
                              HTTP / REST
                                    ▼
                         ┌──────────────────────┐
                         │   FastAPI ML Service │
                         │      Port 8000       │
                         └──────────────────────┘
```

### Service Layout

```text
CFP Publishing System
├── backend/        → Rust + Actix-web API
├── frontend/       → SvelteKit dashboard
├── ml-service/     → Python + FastAPI ML service
├── PostgreSQL      → Persistent storage
├── Redis           → Caching
└── Docker Compose  → Infrastructure orchestration
```

---

## ⚡ Quick Start

### Prerequisites

Install:

* Rust 1.70+
* Node.js 18+
* pnpm
* Python 3.10+
* Docker
* Docker Compose
* PostgreSQL 14+ *(optional when using Docker)*

### 1. Clone the Repository

```bash
git clone https://github.com/adnanphp/cfp-publishing-system.git
cd cfp-publishing-system
```

### 2. Start the Complete System

The recommended approach is to use the included management script:

```bash
chmod +x cfp.sh
./cfp.sh start
```

### Manual Startup

#### Start Infrastructure

```bash
docker-compose up -d
```

This starts:

* PostgreSQL
* Redis

#### Start Backend

```bash
cd backend
cargo build --release
cd ..

./backend/target/release/cfp-backend > backend.log 2>&1 &
```

#### Start Frontend

```bash
cd frontend
pnpm install
pnpm run dev > ../frontend.log 2>&1 &
```

#### Start ML Service

```bash
cd ml-service

python3 -m venv venv
source venv/bin/activate

pip install -r requirements.txt

uvicorn src.main:app \
  --host 0.0.0.0 \
  --port 8000 > ../ml-service.log 2>&1 &
```

---

## 🌐 Service URLs

| Service            | URL                              |
| ------------------ | -------------------------------- |
| Frontend Dashboard | `http://localhost:5173`          |
| Backend API        | `http://localhost:3000`          |
| Backend API Docs   | `http://localhost:3000/api/docs` |
| ML Service         | `http://localhost:8000`          |
| ML API Docs        | `http://localhost:8000/docs`     |

---

## 🛠️ Management Commands

The `cfp.sh` script provides a simple interface for managing the complete application:

```bash
./cfp.sh start
./cfp.sh stop
./cfp.sh status
./cfp.sh restart
./cfp.sh logs [service]
./cfp.sh test
```

Examples:

```bash
./cfp.sh logs backend
./cfp.sh logs frontend
./cfp.sh logs ml
```

---

## 📡 API

### Health & Status

```http
GET /health
GET /status
```

### Core Resources

```http
GET /api/members
GET /api/texts
GET /api/authors
GET /api/downloads
GET /api/downloads/stats
```

### Example Health Response

```json
{
  "service": "cfp-backend",
  "status": "healthy",
  "timestamp": "2026-08-22T22:09:57.115Z"
}
```

---

## 🔧 Development

### Backend

```bash
cd backend

cargo run
```

For automatic reloads:

```bash
cargo install cargo-watch
cargo watch -x run
```

### Frontend

```bash
cd frontend

pnpm install
pnpm run dev
```

Open:

```text
http://localhost:5173
```

### ML Service

```bash
cd ml-service

source venv/bin/activate

uvicorn src.main:app --reload
```

Open the interactive API documentation:

```text
http://localhost:8000/docs
```

---

## 🐳 Docker

Start infrastructure:

```bash
docker-compose up -d
```

View logs:

```bash
docker-compose logs -f
```

Stop services:

```bash
docker-compose down
```

### Docker Compose Services

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

> For production deployments, credentials should be supplied through secrets or environment-specific configuration rather than committed configuration files.

---

## 🧪 Testing

Run the complete endpoint test suite:

```bash
./cfp.sh test
```

Run Rust tests:

```bash
cd backend
cargo test
```

Check the ML service:

```bash
curl http://localhost:8000/health
```

---

## 📁 Project Structure

```text
cfp-publishing-system/
│
├── backend/
│   ├── src/
│   │   ├── api/
│   │   │   └── # HTTP handlers & routes
│   │   ├── domain/
│   │   │   └── # Business logic
│   │   └── main.rs
│   └── Cargo.toml
│
├── frontend/
│   ├── src/
│   │   └── routes/
│   │       └── # SvelteKit pages
│   └── package.json
│
├── ml-service/
│   ├── src/
│   │   └── main.py
│   └── requirements.txt
│
├── docker-compose.yml
├── cfp.sh
└── README.md
```

---

## 🧰 Technology Stack

| Layer             | Technology             |
| ----------------- | ---------------------- |
| Backend           | Rust, Actix-web        |
| Frontend          | SvelteKit, Vite        |
| ML Service        | Python, FastAPI        |
| Database          | PostgreSQL             |
| Cache             | Redis                  |
| Communication     | REST, WebSockets       |
| Containers        | Docker, Docker Compose |
| API Documentation | OpenAPI / Swagger      |

---

## 🔐 Environment Configuration

Create a `.env` file for local development:

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/cfp_db
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
```

**Do not commit `.env` files or production credentials to Git.**

Recommended `.gitignore` entries:

```gitignore
.env
.env.*
!.env.example
```

For collaborators, provide a safe template:

```env
DATABASE_URL=
REDIS_URL=
JWT_SECRET=
```

---

## 🔄 Application Workflow

```text
User
  │
  ▼
SvelteKit Dashboard
  │
  ├────────────── REST API ──────────────┐
  │                                      ▼
  │                              Rust / Actix-web
  │                                      │
  │                         ┌────────────┴────────────┐
  │                         ▼                         ▼
  │                    PostgreSQL                   Redis
  │
  └──────────── WebSocket Notifications

Rust Backend
      │
      └──────────── HTTP ────────────► FastAPI ML Service
                                            │
                                            ▼
                                      Text Analysis
```

---

## 🎯 Engineering Highlights

This project demonstrates experience with:

* **Full-stack application architecture**
* **Rust backend development**
* **REST API design**
* **Python ML microservices**
* **PostgreSQL data modeling**
* **Redis caching**
* **WebSocket communication**
* **SvelteKit frontend development**
* **Docker-based infrastructure**
* **Service-oriented architecture**
* **API documentation with OpenAPI**
* **Automated development and testing workflows**

---

## 🤝 Contributing

Contributions are welcome.

1. Fork the repository.
2. Create a feature branch:

```bash
git checkout -b feature/amazing-feature
```

3. Commit your changes:

```bash
git commit -m "Add amazing feature"
```

4. Push the branch:

```bash
git push origin feature/amazing-feature
```

5. Open a Pull Request.

---

## 📄 License

This project is licensed under the **MIT License**.

See the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Built with open-source technologies including Rust, Actix-web, SvelteKit, FastAPI, PostgreSQL, Redis, and Docker.

---

<div align="center">

**CFP Publishing System**

Full-Stack • Backend • ML • APIs • Cloud-Native Development

</div>
