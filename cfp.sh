#!/bin/bash
# cfp.sh - Comprehensive CFP System Management Script
# Usage: ./cfp.sh [start|stop|status|restart|logs|test|setup]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKEND_PORT=3000
FRONTEND_PORT=5173
ML_PORT=8000
DOCKER_COMPOSE_FILE="docker-compose.yml"
MAX_WAIT=30

# Helper functions
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_header() {
    echo ""
    echo "========================================"
    echo -e "${GREEN}$1${NC}"
    echo "========================================"
    echo ""
}

check_service() {
    local url=$1
    local name=$2
    local max_attempts=10
    local attempt=1
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s -f -o /dev/null -w "%{http_code}" "$url" 2>/dev/null | grep -q "200\|404\|301\|302"; then
            echo -e "${GREEN}✅${NC} $name is running"
            return 0
        fi
        sleep 2
        attempt=$((attempt + 1))
    done
    
    echo -e "${RED}❌${NC} $name is not responding"
    return 1
}

is_running() {
    local pattern=$1
    pgrep -f "$pattern" > /dev/null 2>&1
    return $?
}

kill_if_running() {
    local pattern=$1
    local name=$2
    if is_running "$pattern"; then
        print_status "Stopping $name..."
        pkill -f "$pattern" 2>/dev/null || true
        sleep 2
        print_success "$name stopped"
    else
        print_status "$name is not running"
    fi
}

setup_frontend() {
    print_status "Setting up frontend..."
    cd "$PROJECT_DIR/frontend"
    
    # Check if package.json exists and has dev script
    if [ -f "package.json" ]; then
        if ! grep -q '"dev"' package.json; then
            print_status "Adding dev script to package.json..."
            # Use sed to add dev script
            sed -i '/"scripts": {/a \    "dev": "vite dev",' package.json
            sed -i '/"scripts": {/a \    "build": "vite build",' package.json
            sed -i '/"scripts": {/a \    "preview": "vite preview",' package.json
        fi
    fi
    
    # Check for package manager
    if command -v pnpm &> /dev/null; then
        PKG_MANAGER="pnpm"
    elif command -v npm &> /dev/null; then
        PKG_MANAGER="npm"
    else
        print_error "No package manager found (npm or pnpm required)"
        return 1
    fi
    
    # Install dependencies if needed
    if [ ! -d "node_modules" ]; then
        print_status "Installing frontend dependencies with $PKG_MANAGER..."
        $PKG_MANAGER install
        if [ $? -ne 0 ]; then
            print_error "Failed to install frontend dependencies"
            return 1
        fi
    fi
    
    # Create necessary config files if missing
    if [ ! -f "svelte.config.js" ]; then
        cat > svelte.config.js << 'EOF'
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter()
    }
};

export default config;
EOF
    fi
    
    if [ ! -f "vite.config.ts" ] && [ ! -f "vite.config.js" ]; then
        cat > vite.config.ts << 'EOF'
import { sveltekit } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()]
});
EOF
    fi
    
    if [ ! -f "app.html" ]; then
        cat > app.html << 'EOF'
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <link rel="icon" href="%sveltekit.assets%/favicon.png" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        %sveltekit.head%
    </head>
    <body data-sveltekit-preload-data="hover">
        <div style="display: contents">%sveltekit.body%</div>
    </body>
</html>
EOF
    fi
    
    # Create a simple home page
    mkdir -p src/routes
    if [ ! -f "src/routes/+page.svelte" ]; then
        cat > src/routes/+page.svelte << 'EOF'
<script>
    import { onMount } from 'svelte';
    let backendStatus = 'Checking...';
    let mlStatus = 'Checking...';
    
    onMount(async () => {
        try {
            const response = await fetch('http://localhost:3000/health');
            backendStatus = response.ok ? '✅ Connected' : '❌ Error';
        } catch {
            backendStatus = '❌ Not connected';
        }
        try {
            const response = await fetch('http://localhost:8000');
            mlStatus = response.ok ? '✅ Connected' : '❌ Error';
        } catch {
            mlStatus = '❌ Not connected';
        }
    });
</script>

<main>
    <h1>CFP Publishing System</h1>
    <div class="status">
        <h2>System Status</h2>
        <p><strong>Backend API:</strong> {backendStatus}</p>
        <p><strong>ML Service:</strong> {mlStatus}</p>
        <p><strong>Frontend:</strong> ✅ Running</p>
    </div>
    <div class="links">
        <h2>Quick Links</h2>
        <ul>
            <li><a href="http://localhost:3000/api/docs" target="_blank">Backend API Documentation</a></li>
            <li><a href="http://localhost:8000/docs" target="_blank">ML Service Documentation</a></li>
        </ul>
    </div>
</main>

<style>
    main { max-width: 800px; margin: 0 auto; padding: 2rem; font-family: system-ui, sans-serif; }
    h1 { color: #2563eb; margin-bottom: 2rem; }
    .status, .links { background: #f8fafc; padding: 1.5rem; border-radius: 8px; margin-bottom: 1.5rem; border: 1px solid #e2e8f0; }
    h2 { color: #475569; margin-top: 0; }
    ul { padding-left: 1.5rem; }
    li { margin-bottom: 0.5rem; }
    a { color: #2563eb; text-decoration: none; }
    a:hover { text-decoration: underline; }
</style>
EOF
    fi
    
    cd "$PROJECT_DIR"
    print_success "Frontend setup complete"
    return 0
}

wait_for_frontend() {
    local max_attempts=30
    local attempt=1
    
    print_status "Waiting for frontend to be ready..."
    while [ $attempt -le $max_attempts ]; do
        if curl -s -f -o /dev/null "http://localhost:$FRONTEND_PORT" 2>/dev/null; then
            print_success "Frontend is ready"
            return 0
        fi
        
        # Check if the process is still running
        if ! is_running "vite"; then
            print_warning "Frontend process died, showing full log:"
            cat "$PROJECT_DIR/frontend.log" 2>/dev/null || echo "No log file found"
            return 1
        fi
        
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    print_error "Frontend failed to start within $max_attempts attempts"
    print_status "Full frontend log:"
    cat "$PROJECT_DIR/frontend.log" 2>/dev/null || echo "No log file found"
    return 1
}

start_services() {
    local rebuild=${1:-false}
    print_header "🚀 Starting CFP Publishing System"
    
    # Stop existing services first
    stop_services false
    
    # Start Docker services
    print_status "Starting Docker services..."
    if [ -f "$PROJECT_DIR/$DOCKER_COMPOSE_FILE" ]; then
        cd "$PROJECT_DIR"
        docker-compose up -d
        if [ $? -eq 0 ]; then
            print_success "Docker services started"
            sleep 5
        else
            print_error "Failed to start Docker services"
            exit 1
        fi
    else
        print_warning "Docker compose file not found, skipping Docker"
    fi
    
    # Start Backend
    print_status "Starting Backend service..."
    cd "$PROJECT_DIR/backend"
    
    # Build if needed
    if [ ! -f "target/release/cfp-backend" ] || [ "$rebuild" = "true" ]; then
        print_status "Building backend..."
        cargo build --release
        if [ $? -ne 0 ]; then
            print_error "Backend build failed"
            exit 1
        fi
        print_success "Backend built successfully"
    fi
    
    cd "$PROJECT_DIR"
    nohup ./backend/target/release/cfp-backend > backend.log 2>&1 &
    BACKEND_PID=$!
    print_success "Backend started (PID: $BACKEND_PID)"
    sleep 3
    
    # Check if backend is responding
    if ! check_service "http://localhost:$BACKEND_PORT/health" "Backend API"; then
        print_error "Backend failed to start properly"
        tail -10 backend.log
        exit 1
    fi
    
    # Setup Frontend
    print_status "Setting up Frontend..."
    if ! setup_frontend; then
        print_error "Frontend setup failed"
        exit 1
    fi
    
    # Start Frontend directly with the correct command
    print_status "Starting Frontend service..."
    cd "$PROJECT_DIR/frontend"
    
    # Check for package manager
    if command -v pnpm &> /dev/null; then
        PKG_MANAGER="pnpm"
    elif command -v npm &> /dev/null; then
        PKG_MANAGER="npm"
    else
        print_error "No package manager found (npm or pnpm required)"
        exit 1
    fi
    
    # Start frontend directly - use npm/pnpm run dev
    cd "$PROJECT_DIR"
    nohup bash -c "cd frontend && $PKG_MANAGER run dev -- --host 0.0.0.0" > frontend.log 2>&1 &
    FRONTEND_PID=$!
    print_success "Frontend started (PID: $FRONTEND_PID)"
    
    # Wait for frontend to be ready
    if ! wait_for_frontend; then
        print_warning "Frontend may not be working properly"
        print_status "Check the full log with: cat frontend.log"
    fi
    
    # Start ML Service
    print_status "Starting ML Service..."
    cd "$PROJECT_DIR/ml-service"
    
    # Setup virtual environment if needed
    if [ ! -d "venv" ]; then
        print_status "Creating Python virtual environment..."
        python3 -m venv venv
    fi
    
    # Activate and install requirements
    source venv/bin/activate
    if [ -f "requirements.txt" ]; then
        pip install -r requirements.txt > /dev/null 2>&1 || pip install fastapi uvicorn
    else
        pip install fastapi uvicorn > /dev/null 2>&1
    fi
    
    cd "$PROJECT_DIR"
    nohup uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload --app-dir ml-service > ml-service.log 2>&1 &
    ML_PID=$!
    print_success "ML Service started (PID: $ML_PID)"
    sleep 3
    
    # Save PIDs
    cat > .cfp_pids << EOF
BACKEND_PID=$BACKEND_PID
FRONTEND_PID=$FRONTEND_PID
ML_PID=$ML_PID
EOF
    
    # Final status check
    print_header "🔍 Final Service Status"
    
    check_service "http://localhost:$BACKEND_PORT/health" "Backend API"
    check_service "http://localhost:$FRONTEND_PORT" "Frontend"
    check_service "http://localhost:$ML_PORT" "ML Service"
    
    print_header "✅ CFP System Started!"
    echo ""
    echo "📋 Service URLs:"
    echo "  Frontend:     http://localhost:$FRONTEND_PORT"
    echo "  Backend API:  http://localhost:$BACKEND_PORT"
    echo "  Backend Docs: http://localhost:$BACKEND_PORT/api/docs"
    echo "  ML Service:   http://localhost:$ML_PORT"
    echo "  ML Docs:      http://localhost:$ML_PORT/docs"
    echo ""
    echo "📝 Logs:"
    echo "  Backend:  tail -f $PROJECT_DIR/backend.log"
    echo "  Frontend: tail -f $PROJECT_DIR/frontend.log"
    echo "  ML:       tail -f $PROJECT_DIR/ml-service.log"
    echo ""
    echo "🔧 Management:"
    echo "  Status:  ./cfp.sh status"
    echo "  Stop:    ./cfp.sh stop"
    echo "  Restart: ./cfp.sh restart"
    echo "  Logs:    ./cfp.sh logs [backend|frontend|ml]"
    echo "  Test:    ./cfp.sh test"
    echo ""
}

stop_services() {
    local show_header=${1:-true}
    if [ "$show_header" = true ]; then
        print_header "🛑 Stopping CFP System"
    fi
    
    if [ -f ".cfp_pids" ]; then
        source .cfp_pids
    fi
    
    kill_if_running "cfp-backend" "Backend"
    if [ ! -z "$BACKEND_PID" ]; then
        kill -9 $BACKEND_PID 2>/dev/null || true
    fi
    
    kill_if_running "vite" "Frontend"
    if [ ! -z "$FRONTEND_PID" ]; then
        kill -9 $FRONTEND_PID 2>/dev/null || true
    fi
    
    kill_if_running "uvicorn src.main:app" "ML Service"
    if [ ! -z "$ML_PID" ]; then
        kill -9 $ML_PID 2>/dev/null || true
    fi
    
    print_status "Stopping Docker services..."
    cd "$PROJECT_DIR"
    docker-compose down 2>/dev/null || true
    print_success "Docker services stopped"
    
    rm -f .cfp_pids
    
    if [ "$show_header" = true ]; then
        print_success "✅ All services stopped"
    fi
}

status_services() {
    print_header "📊 CFP System Status"
    
    echo "🐳 Docker Status:"
    if docker ps --format "table {{.Names}}\t{{.Status}}" 2>/dev/null | grep -q "cfp"; then
        docker ps --format "table {{.Names}}\t{{.Status}}" | grep "cfp"
    else
        echo "  No CFP Docker containers running"
    fi
    echo ""
    
    echo "📋 Service Status:"
    check_service "http://localhost:$BACKEND_PORT/health" "Backend (port $BACKEND_PORT)"
    check_service "http://localhost:$FRONTEND_PORT" "Frontend (port $FRONTEND_PORT)"
    check_service "http://localhost:$ML_PORT" "ML Service (port $ML_PORT)"
    echo ""
    
    echo "🔍 Process Details:"
    if is_running "cfp-backend"; then
        echo "  Backend:   $(pgrep -f cfp-backend | head -1)"
    else
        echo "  Backend:   Not running"
    fi
    if is_running "vite"; then
        echo "  Frontend:  $(pgrep -f vite | head -1)"
    else
        echo "  Frontend:  Not running"
    fi
    if is_running "uvicorn src.main:app"; then
        echo "  ML:        $(pgrep -f uvicorn src.main:app | head -1)"
    else
        echo "  ML:        Not running"
    fi
    echo ""
    
    echo "📝 Recent Logs:"
    [ -f "backend.log" ] && echo "  Backend:  $(tail -1 backend.log 2>/dev/null | cut -c1-80)"
    [ -f "frontend.log" ] && echo "  Frontend: $(tail -1 frontend.log 2>/dev/null | cut -c1-80)"
    [ -f "ml-service.log" ] && echo "  ML:       $(tail -1 ml-service.log 2>/dev/null | cut -c1-80)"
    echo ""
}

view_logs() {
    local service=$1
    
    print_header "📝 Viewing Logs"
    
    case "$service" in
        backend|back)
            if [ -f "backend.log" ]; then
                tail -50 backend.log
            else
                print_error "backend.log not found"
            fi
            ;;
        frontend|front)
            if [ -f "frontend.log" ]; then
                cat frontend.log
            else
                print_error "frontend.log not found"
            fi
            ;;
        ml|ml-service)
            if [ -f "ml-service.log" ]; then
                tail -50 ml-service.log
            else
                print_error "ml-service.log not found"
            fi
            ;;
        all|*)
            echo "=== Backend Logs ==="
            [ -f "backend.log" ] && tail -20 backend.log || echo "No backend.log"
            echo ""
            echo "=== Frontend Logs ==="
            [ -f "frontend.log" ] && cat frontend.log || echo "No frontend.log"
            echo ""
            echo "=== ML Service Logs ==="
            [ -f "ml-service.log" ] && tail -20 ml-service.log || echo "No ml-service.log"
            ;;
    esac
}

test_endpoints() {
    print_header "🧪 Testing All Endpoints"
    
    BASE_URL="http://localhost:$BACKEND_PORT"
    endpoints=(
        "/"
        "/health"
        "/status"
        "/api"
        "/api/health"
        "/api/status"
        "/api/db-test"
        "/api/members"
        "/api/authors"
        "/api/texts"
        "/api/downloads"
        "/api/downloads/count"
        "/api/downloads/stats"
        "/api/downloads/top-texts"
    )
    
    for endpoint in "${endpoints[@]}"; do
        url="$BASE_URL$endpoint"
        echo -n "🔍 $url: "
        
        if timeout 3 curl -s -f "$url" > /dev/null 2>&1; then
            echo -e "${GREEN}✅ LIVE${NC}"
            response=$(timeout 3 curl -s "$url" 2>/dev/null | head -2)
            if [ ! -z "$response" ]; then
                echo "   📦 $(echo "$response" | tr '\n' ' ' | cut -c1-80)..."
            fi
        else
            echo -e "${RED}❌ DOWN or ERROR${NC}"
        fi
    done
    
    echo ""
    echo "========================================"
    echo "Note: Some endpoints may require POST data or authentication"
}

restart_services() {
    print_header "🔄 Restarting CFP System"
    stop_services false
    sleep 3
    start_services false
}

# Main script
case "$1" in
    start)
        if [ "$2" = "rebuild" ]; then
            start_services true
        else
            start_services false
        fi
        ;;
    stop)
        stop_services
        ;;
    status|state)
        status_services
        ;;
    restart)
        restart_services
        ;;
    logs|log)
        view_logs "$2"
        ;;
    test)
        test_endpoints
        ;;
    setup)
        setup_frontend
        ;;
    *)
        echo "Usage: $0 {start|stop|status|restart|logs|test|setup}"
        echo ""
        echo "Commands:"
        echo "  start          Start all services"
        echo "  start rebuild  Start services and rebuild backend"
        echo "  stop           Stop all services"
        echo "  status         Show current status of all services"
        echo "  restart        Restart all services"
        echo "  logs [service] View logs (backend|frontend|ml|all)"
        echo "  test           Test all backend endpoints"
        echo "  setup          Setup/fix frontend configuration"
        echo ""
        echo "Examples:"
        echo "  ./cfp.sh setup         # Fix frontend issues first"
        echo "  ./cfp.sh start         # Start the system"
        echo "  ./cfp.sh status        # Check status"
        echo "  ./cfp.sh logs frontend # See full frontend log"
        echo "  ./cfp.sh test          # Test all endpoints"
        exit 1
        ;;
esac

exit 0
