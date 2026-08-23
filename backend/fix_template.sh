#!/bin/bash

# Fix all template syntax errors at once
find src/infrastructure/database/repositories -name "*.rs" -exec sed -i 's/\${repo\^}RepositoryImpl/&/' {} \; 2>/dev/null

# Actually, let's fix each one specifically:
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct TextRepositoryImpl;/' src/infrastructure/database/repositories/text_repository.rs
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct DonationRepositoryImpl;/' src/infrastructure/database/repositories/donation_repository.rs
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct PlagiarismRepositoryImpl;/' src/infrastructure/database/repositories/plagiarism_repository.rs
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct CommitteeRepositoryImpl;/' src/infrastructure/database/repositories/committee_repository.rs
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct CharityRepositoryImpl;/' src/infrastructure/database/repositories/charity_repository.rs
sed -i 's/pub struct \${repo\^}RepositoryImpl;/pub struct NotificationRepositoryImpl;/' src/infrastructure/database/repositories/notification_repository.rs

echo "Template syntax fixed"
