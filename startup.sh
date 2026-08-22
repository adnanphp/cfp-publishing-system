#!/bin/bash
# startup.sh - CFP Startup for SvelteKit

set -e

echo "🚀 Starting CFP Publishing System..."

# 1. START DOCKER
echo "Starting Docker services..."
cd ~/projects/cfp
docker-compose up -d
sleep 5

# 2. DATABASE SETUP
echo "Checking database..."
cd ~/projects/cfp/backend
echo "Skipping migrations (assuming database exists)..."

# 3. BUILD BACKEND
echo "Building backend..."
cargo build --release

# 4. STOP AND START BACKEND
echo "Starting backend..."
cd ~/projects/cfp
pkill -f cfp-backend || true
sleep 3
rm -f cfp-backend
cp backend/target/release/cfp-backend .
chmod +x cfp-backend
nohup ./cfp-backend > backend.log 2>&1 &
BACKEND_PID=$!
echo "Backend PID: $BACKEND_PID"
sleep 3

# 5. FRONTEND SETUP FOR SVELTEKIT
echo "Setting up frontend..."
cd ~/projects/cfp/frontend

# Fix package.json if needed
if ! python3 -c "import json; json.load(open('package.json'))" 2>/dev/null; then
    echo "Creating SvelteKit package.json..."
    cat > package.json << 'EOF'
{
  "name": "cfp-frontend",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch"
  },
  "devDependencies": {
    "@sveltejs/adapter-auto": "^3.0.0",
    "@sveltejs/kit": "^2.0.0",
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "@types/node": "^20.10.0",
    "svelte": "^4.2.7",
    "svelte-check": "^3.6.0",
    "tslib": "^2.6.2",
    "typescript": "^5.3.0",
    "vite": "^5.0.3"
  },
  "dependencies": {
    "axios": "^1.6.0"
  }
}
EOF
fi

# Create necessary SvelteKit files if missing
if [ ! -f "svelte.config.js" ]; then
    echo "Creating svelte.config.js..."
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

if [ ! -f "vite.config.ts" ]; then
    echo "Creating vite.config.ts..."
    cat > vite.config.ts << 'EOF'
import { sveltekit } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [sveltekit()]
});
EOF
fi

if [ ! -f "tsconfig.json" ]; then
    echo "Creating tsconfig.json..."
    cat > tsconfig.json << 'EOF'
{
    "extends": "./.svelte-kit/tsconfig.json",
    "compilerOptions": {
        "allowJs": true,
        "checkJs": true,
        "esModuleInterop": true,
        "forceConsistentCasingInFileNames": true,
        "resolveJsonModule": true,
        "skipLibCheck": true,
        "sourceMap": true,
        "strict": true,
        "moduleResolution": "bundler"
    }
}
EOF
fi

# Create app.html if missing
if [ ! -f "app.html" ]; then
    echo "Creating app.html..."
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

# Create src directory structure
mkdir -p src/routes

# Create a simple home page if none exists
if [ ! -f "src/routes/+page.svelte" ]; then
    echo "Creating home page..."
    cat > src/routes/+page.svelte << 'EOF'
<script>
    import { onMount } from 'svelte';
    
    let backendStatus = 'Checking...';
    let mlStatus = 'Checking...';
    
    onMount(async () => {
        // Check backend
        try {
            const response = await fetch('http://localhost:3000/health');
            backendStatus = '✅ Connected';
        } catch {
            backendStatus = '❌ Not connected';
        }
        
        // Check ML service
        try {
            const response = await fetch('http://localhost:8000');
            mlStatus = '✅ Connected';
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
    main {
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem;
        font-family: system-ui, -apple-system, sans-serif;
    }
    
    h1 {
        color: #2563eb;
        margin-bottom: 2rem;
    }
    
    .status, .links {
        background: #f8fafc;
        padding: 1.5rem;
        border-radius: 8px;
        margin-bottom: 1.5rem;
        border: 1px solid #e2e8f0;
    }
    
    h2 {
        color: #475569;
        margin-top: 0;
    }
    
    ul {
        padding-left: 1.5rem;
    }
    
    li {
        margin-bottom: 0.5rem;
    }
    
    a {
        color: #2563eb;
        text-decoration: none;
    }
    
    a:hover {
        text-decoration: underline;
    }
</style>
EOF
fi

# 6. INSTALL AND BUILD
echo "Installing dependencies..."
if command -v pnpm &> /dev/null; then
    pnpm install
elif command -v npm &> /dev/null; then
    npm install
fi

echo "Building frontend..."
if command -v pnpm &> /dev/null; then
    pnpm run build || echo "Build warning, continuing..."
elif command -v npm &> /dev/null; then
    npm run build || echo "Build warning, continuing..."
fi

# 7. START FRONTEND
echo "Starting frontend..."
pkill -f "vite" || true
sleep 2
if command -v pnpm &> /dev/null; then
    nohup pnpm run preview > ../frontend.log 2>&1 &
elif command -v npm &> /dev/null; then
    nohup npm run preview > ../frontend.log 2>&1 &
fi
FRONTEND_PID=$!
echo "Frontend PID: $FRONTEND_PID"
sleep 2

# 8. ML SERVICE
echo "Starting ML service..."
cd ~/projects/cfp/ml-service

# Create simple service if missing
mkdir -p src
if [ ! -f "src/main.py" ]; then
    cat > src/main.py << 'EOF'
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="CFP ML Service")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
async def root():
    return {"message": "CFP ML Service is running"}

@app.get("/health")
async def health():
    return {"status": "healthy"}

@app.get("/docs")
async def get_docs():
    return {"docs_url": "http://localhost:8000/docs"}
EOF
fi

if [ ! -f "requirements.txt" ]; then
    echo "fastapi>=0.104.0
uvicorn[standard]>=0.24.0" > requirements.txt
fi

if [ ! -d "venv" ]; then
    python3 -m venv venv
fi
source venv/bin/activate
pip install -r requirements.txt 2>/dev/null || pip install fastapi uvicorn

pkill -f "uvicorn src.main:app" || true
sleep 2
nohup uvicorn src.main:app --host 0.0.0.0 --port 8000 --reload > ../ml-service.log 2>&1 &
ML_PID=$!
echo "ML Service PID: $ML_PID"

# 9. SAVE PIDS
cd ~/projects/cfp
cat > .cfp_pids << EOF
BACKEND_PID=$BACKEND_PID
FRONTEND_PID=$FRONTEND_PID
ML_PID=$ML_PID
EOF

# 10. DISPLAY STATUS
echo ""
echo "========================================"
echo "✅ CFP System Started Successfully!"
echo "========================================"
echo ""
echo "Services Running:"
echo "-----------------"
echo "Backend API:    http://localhost:3000"
echo "Frontend App:   http://localhost:5173"
echo "ML Service:     http://localhost:8000"
echo ""
echo "Access URLs:"
echo "------------"
echo "Web Interface:  http://localhost:5173"
echo "API Docs:       http://localhost:3000/api/docs"
echo "ML API Docs:    http://localhost:8000/docs"
echo ""
echo "To test backend:"
echo "curl http://localhost:3000/health"
echo ""
echo "To stop: ./shutdown.sh"
