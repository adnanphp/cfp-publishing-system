#!/bin/bash

FILE="src/api/handlers/text_handler.rs"

# Create a backup
cp "$FILE" "${FILE}.backup2"

# Look for the TextCommentResponse struct
TEXT="pub struct TextCommentResponse {"
if grep -q "$TEXT" "$FILE"; then
    echo "Found TextCommentResponse struct"
    
    # Find the line number
    LINE=$(grep -n "$TEXT" "$FILE" | head -1 | cut -d: -f1)
    echo "Struct starts at line: $LINE"
    
    # Extract from this line to find where it should end
    # Let's look for the next } at the same indentation level
    awk -v start="$LINE" '
    NR >= start {
        print NR ": " $0
        if (NR > start && /^[[:space:]]*}[[:space:]]*$/ && brace_count == 0) {
            print "Found closing brace at line: " NR
            exit
        }
        if (/{/) brace_count++
        if (/}/) brace_count--
    }' "$FILE" > /tmp/debug.txt
    
    # Let's just add a closing brace if missing at the end
    # First, check if the file ends properly
    LAST_CHAR=$(tail -c 1 "$FILE" | od -An -t x1 | tr -d ' \n')
    if [ "$LAST_CHAR" != "7d" ]; then  # 7d is ASCII for }
        echo "File doesn't end with '}'. Adding it."
        echo "}" >> "$FILE"
    fi
    
    # Fix duplicate replies field by commenting out the i32 version
    sed -i 's/^    pub replies: i32,/    \/\/ pub replies: i32,  \/\/ Duplicate field/' "$FILE"
fi

echo "Text handler fixed"
