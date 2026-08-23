#!/bin/bash

# Create a clean section around the problematic area
FILE="src/api/handlers/text_handler.rs"

# Extract the part before the problematic struct
head -n 530 "$FILE" > /tmp/text_part1.txt

# Create the fixed struct section
cat > /tmp/text_struct_fixed.txt << 'STRUCTFIX'
pub struct TextCommentResponse {
    pub comment_id: Uuid,
    pub text_id: Uuid,
    pub member_id: Uuid,
    pub member_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub replies: Vec<ReplyCommentResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyCommentResponse {
    pub reply_id: Uuid,
    pub comment_id: Uuid,
    pub member_id: Uuid,
    pub member_name: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
STRUCTFIX

# Extract the part after line 580 (skip the broken middle part)
tail -n +580 "$FILE" > /tmp/text_part2.txt

# Combine them
cat /tmp/text_part1.txt /tmp/text_struct_fixed.txt /tmp/text_part2.txt > "$FILE.new"

# Replace the original
mv "$FILE.new" "$FILE"

echo "Manually fixed text_handler.rs structs"
