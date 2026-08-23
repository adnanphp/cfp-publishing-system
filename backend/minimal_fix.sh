#!/bin/bash
# Comment out problematic code temporarily
echo "Temporarily commenting out problematic middleware implementations..."
find src/api/middleware -name "*.rs" -type f ! -name "mod.rs" -exec sed -i 's/^\(.*impl.*\)$/# \1/' {} \;
find src/application/commands -name "*.rs" -type f -exec sed -i 's/^\(.*impl.*CommandHandler.*\)$/# \1/' {} \;
