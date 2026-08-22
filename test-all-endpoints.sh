#!/bin/bash
echo "🧪 Testing ALL CFP Backend Endpoints"
echo "===================================="

BASE_URL="http://localhost:3000"
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
    "/api/auth/login"
)

for endpoint in "${endpoints[@]}"; do
    url="$BASE_URL$endpoint"
    echo -n "🔍 $url: "
    
    # Use timeout to avoid hanging
    if timeout 3 curl -s -f "$url" > /dev/null; then
        echo "✅ LIVE"
        # Show a snippet of successful responses
        if [[ "$endpoint" != "/api/auth/login" ]]; then
            response=$(timeout 3 curl -s "$url")
            if echo "$response" | grep -q "{"; then
                echo "   📦 $(echo "$response" | head -2 | tr '\n' ' ' | cut -c1-80)..."
            else
                echo "   📦 Response received"
            fi
        fi
    else
        echo "❌ DOWN or ERROR"
    fi
    echo ""
done

echo "===================================="
echo "Note: Some endpoints may require POST data or authentication"
