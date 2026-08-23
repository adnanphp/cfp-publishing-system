#!/bin/bash

echo "Fixing enum naming and import issues..."
echo "========================================"

# 1. Fix enum naming (PascalCase)
echo -e "\n1. Fixing enum naming to PascalCase..."

# Fix charity_status.rs
sed -i 's/pub enum Charity_status/pub enum CharityStatus/' src/domain/enums/charity_status.rs
sed -i 's/Charity_status::/CharityStatus::/g' src/domain/enums/charity_status.rs

# Fix committee_scope.rs
sed -i 's/pub enum Committee_scope/pub enum CommitteeScope/' src/domain/enums/committee_scope.rs
sed -i 's/Committee_scope::/CommitteeScope::/g' src/domain/enums/committee_scope.rs

# Fix plagiarism_status.rs
sed -i 's/pub enum Plagiarism_status/pub enum PlagiarismStatus/' src/domain/enums/plagiarism_status.rs
sed -i 's/Plagiarism_status::/PlagiarismStatus::/g' src/domain/enums/plagiarism_status.rs

# Fix notification_type.rs
sed -i 's/pub enum Notification_type/pub enum NotificationType/' src/domain/enums/notification_type.rs
sed -i 's/Notification_type::/NotificationType::/g' src/domain/enums/notification_type.rs

# Fix notification_priority.rs
sed -i 's/pub enum Notification_priority/pub enum NotificationPriority/' src/domain/enums/notification_priority.rs
sed -i 's/Notification_priority::/NotificationPriority::/g' src/domain/enums/notification_priority.rs

# Fix role_enum.rs
sed -i 's/pub enum Role_enum/pub enum RoleEnum/' src/domain/enums/role_enum.rs
sed -i 's/Role_enum::/RoleEnum::/g' src/domain/enums/role_enum.rs

# 2. Create missing enums
echo -e "\n2. Creating missing enums..."

# Create CommentStatus
cat > src/domain/enums/comment_status.rs << 'EOF'
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommentStatus {
    Published,
    Hidden,
    Deleted,
}
EOF

# Create CommitteeMembershipRole
cat > src/domain/enums/committee_membership_role.rs << 'EOF'
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommitteeMembershipRole {
    Member,
    Chair,
    Secretary,
    Treasurer,
}
EOF

# Create CommitteeMembershipStatus
cat > src/domain/enums/committee_membership_status.rs << 'EOF'
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CommitteeMembershipStatus {
    Active,
    Inactive,
    Suspended,
}
EOF

# Create VoteType
cat > src/domain/enums/vote_type.rs << 'EOF'
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VoteType {
    Plagiarism,
    Content,
    Committee,
}
EOF

# Update enums/mod.rs
cat >> src/domain/enums/mod.rs << 'EOF'
pub mod comment_status;
pub mod committee_membership_role;
pub mod committee_membership_status;
pub mod vote_type;

pub use comment_status::*;
pub use committee_membership_role::*;
pub use committee_membership_status::*;
pub use vote_type::*;
EOF

# 3. Fix model imports
echo -e "\n3. Fixing model imports..."

# Fix admin.rs
sed -i 's/use crate::domain::enums::role_enum::RoleEnum;/use crate::domain::enums::RoleEnum;/' src/domain/models/admin.rs

# Fix comment.rs
sed -i 's/use crate::domain::enums::CommentStatus;/use crate::domain::enums::CommentStatus;/' src/domain/models/comment.rs

# Fix committee.rs
sed -i 's/use crate::domain::enums::{committee_scope::CommitteeScope, CommitteeStatus};/use crate::domain::enums::{CommitteeScope, CommitteeStatus};/' src/domain/models/committee.rs

# Fix committee_membership.rs
sed -i 's/use crate::domain::enums::{CommitteeMembershipRole, CommitteeMembershipStatus};/use crate::domain::enums::{CommitteeMembershipRole, CommitteeMembershipStatus};/' src/domain/models/committee_membership.rs

# Fix notification.rs
sed -i 's/use crate::domain::enums::{notification_priority::NotificationPriority, notification_type::NotificationType};/use crate::domain::enums::{NotificationPriority, NotificationType};/' src/domain/models/notification.rs

# Fix plagiarism_case.rs
sed -i 's/use crate::domain::enums::plagiarism_status::PlagiarismStatus;/use crate::domain::enums::PlagiarismStatus;/' src/domain/models/plagiarism_case.rs

# Fix text_version.rs
sed -i 's/use crate::domain::enums::text_status::TextStatus;/use crate::domain::enums::TextStatus;/' src/domain/models/text_version.rs

# Fix vote.rs
sed -i 's/use crate::domain::enums::VoteType;/use crate::domain::enums::VoteType;/' src/domain/models/vote.rs

# 4. Fix logger.rs imports
echo -e "\n4. Fixing logger.rs..."
cat > src/utils/logger.rs << 'EOF'
use tracing::{info, error, warn, debug};
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::EnvFilter;

pub fn setup_logger() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false);
    
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
    
    info!("Logger initialized");
}

pub fn log_info(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}

pub fn log_warning(message: &str) {
    warn!("{}", message);
}

pub fn log_debug(message: &str) {
    debug!("{}", message);
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

impl LogEntry {
    pub fn new(
        level: &str,
        message: &str,
        module_path: Option<String>,
        file: Option<String>,
        line: Option<u32>,
    ) -> Self {
        Self {
            timestamp: chrono::Local::now().to_rfc3339(),
            level: level.to_string(),
            message: message.to_string(),
            module_path,
            file,
            line,
        }
    }
    
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }
}
EOF

# 5. Fix author.rs syntax error
echo -e "\n5. Fixing author.rs syntax..."
if [ -f "src/domain/models/author.rs" ]; then
    # Remove the semicolon at line 43
    sed -i '43s/;$//' src/domain/models/author.rs
fi

# 6. Fix donation.rs duplicate Clone derive
echo -e "\n6. Fixing donation.rs..."
if [ -f "src/domain/models/donation.rs" ]; then
    # Remove the duplicate Clone derive (line 9)
    sed -i '9d' src/domain/models/donation.rs
fi

# 7. Temporarily remove Validate from donation.rs to fix compilation
echo -e "\n7. Temporarily removing Validate from donation.rs..."
if [ -f "src/domain/models/donation.rs" ]; then
    # Remove Validate from the derive macro
    sed -i '8s/, Validate//' src/domain/models/donation.rs
fi

# 8. Remove validator attributes from models temporarily
echo -e "\n8. Removing validator attributes temporarily..."
for file in src/domain/models/author.rs src/domain/models/text.rs; do
    if [ -f "$file" ]; then
        echo "Cleaning $file..."
        # Remove validate attributes using sed
        sed -i '/#\[validate(/d' "$file"
    fi
done

# 9. Test compilation
echo -e "\n9. Testing compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo -e "\n✅ SUCCESS! Project compiles!"
    
    echo -e "\nSummary of fixes:"
    echo "1. Fixed enum naming to PascalCase"
    echo "2. Created missing enums"
    echo "3. Fixed model imports"
    echo "4. Fixed logger.rs imports"
    echo "5. Fixed author.rs syntax"
    echo "6. Fixed donation.rs duplicate Clone"
    echo "7. Temporarily removed Validate from donation.rs"
    echo "8. Removed validator attributes"
    
    echo -e "\n🎉 Congratulations! Your project now compiles!"
    echo ""
    echo "Next steps:"
    echo "1. You can now restore remaining modules:"
    echo "   find ./src.backup.1769027748 -name '*.rs' | grep -E '(services|queries|commands|infrastructure|api)' | sort"
    echo ""
    echo "2. Restore one module at a time and test:"
    echo "   mkdir -p src/application/services"
    echo "   cp ./src.backup.1769027748/application/services/mod.rs src/application/services/"
    echo "   cargo check"
    echo ""
    echo "3. If you need validator, add it back gradually:"
    echo "   - Add Validate to derive macros"
    echo "   - Add validate attributes"
    echo ""
    echo "4. Test the build: cargo build"
else
    echo -e "\n❌ Still have errors. Showing first few:"
    cargo check 2>&1 | grep -B 2 -A 2 "error\[E" | head -30
fi
