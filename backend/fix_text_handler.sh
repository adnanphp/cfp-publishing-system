#!/bin/bash

# Find the problematic struct and fix it
FILE="src/api/handlers/text_handler.rs"

# Backup the file
cp "$FILE" "${FILE}.backup"

# Find the TextCommentResponse struct and ensure it's closed properly
# Look for the struct definition and its closing brace
START_LINE=$(grep -n "pub struct TextCommentResponse" "$FILE" | cut -d: -f1)
if [ -n "$START_LINE" ]; then
    # Find the next struct or impl after this one
    END_LINE=$(awk -v start="$START_LINE" 'NR > start && /^pub struct |^impl|^pub fn/ {print NR; exit}' "$FILE")
    
    if [ -z "$END_LINE" ]; then
        END_LINE=$(wc -l < "$FILE")
    fi
    
    # Extract the struct content
    sed -n "${START_LINE},${END_LINE}p" "$FILE" > /tmp/struct_content.txt
    
    # Count braces
    OPEN_BRACES=$(grep -o "{" /tmp/struct_content.txt | wc -l)
    CLOSE_BRACES=$(grep -o "}" /tmp/struct_content.txt | wc -l)
    
    echo "Struct at line $START_LINE"
    echo "Open braces: $OPEN_BRACES"
    echo "Close braces: $CLOSE_BRACES"
    
    if [ "$OPEN_BRACES" -gt "$CLOSE_BRACES" ]; then
        echo "Missing close brace detected. Adding closing brace..."
        
        # Insert closing brace before the next struct/impl
        LINE_TO_INSERT=$((END_LINE - 1))
        sed -i "${LINE_TO_INSERT}a\\}" "$FILE"
    fi
fi

# Also check for duplicate 'replies' field around line 544
sed -i '540,550s/pub replies: i32,/# pub replies: i32,/' "$FILE"

echo "Fixed text_handler.rs"
