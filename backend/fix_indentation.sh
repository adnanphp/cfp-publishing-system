#!/bin/bash

echo "Fixing indentation error in plagiarism_queries.rs..."

# First, let's look at the problematic section
echo "Original lines 9-20:"
sed -n '9,20p' src/application/queries/plagiarism_queries.rs

# Fix the indentation issue
sed -i '9,20d' src/application/queries/plagiarism_queries.rs

# Replace with properly formatted imports
cat > /tmp/plagiarism_fix.txt << 'EOF'
        dto::{
            PlagiarismCaseResponse,
            PlagiarismCaseSearchResponse,
            VoteResponse,
            PlagiarismStatsResponse,
        },
EOF

# Insert the fixed content
sed -i '8r /tmp/plagiarism_fix.txt' src/application/queries/plagiarism_queries.rs

echo "Fixed content:"
sed -n '8,13p' src/application/queries/plagiarism_queries.rs

echo "Running cargo check..."
cargo check
