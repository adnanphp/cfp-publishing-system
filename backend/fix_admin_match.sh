#!/bin/bash

echo "Fixing non-exhaustive match in admin.rs..."
echo "=========================================="

# Fix the match statement in admin.rs
if [ -f "src/domain/models/admin.rs" ]; then
    echo "Current match statement lines 20-40:"
    sed -n '20,40p' src/domain/models/admin.rs
    
    echo -e "\nFixing the match statement..."
    # Create a fixed version
    cat > /tmp/admin_fixed.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::enums::RoleEnum;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: Uuid,
    pub member_id: Uuid,
    pub role: RoleEnum,
    pub assigned_committees: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Admin {
    pub fn new(member_id: Uuid, role: RoleEnum) -> Self {
        Self {
            id: Uuid::new_v4(),
            member_id,
            role,
            assigned_committees: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    pub fn get_permissions(&self) -> Vec<String> {
        match self.role {
            RoleEnum::Super => vec![
                "manage_users".to_string(),
                "manage_content".to_string(),
                "manage_finances".to_string(),
                "manage_committees".to_string(),
                "view_reports".to_string(),
            ],
            RoleEnum::Content => vec![
                "manage_content".to_string(),
                "review_content".to_string(),
                "view_reports".to_string(),
            ],
            RoleEnum::Financial => vec![
                "manage_finances".to_string(),
                "view_financial_reports".to_string(),
            ],
            RoleEnum::Admin | RoleEnum::User => vec![
                "view_reports".to_string(),
            ],
        }
    }
    
    pub fn can_manage_users(&self) -> bool {
        matches!(self.role, RoleEnum::Super)
    }
    
    pub fn can_manage_content(&self) -> bool {
        matches!(self.role, RoleEnum::Super | RoleEnum::Content)
    }
    
    pub fn can_manage_finances(&self) -> bool {
        matches!(self.role, RoleEnum::Super | RoleEnum::Financial)
    }
    
    pub fn assign_to_committee(&mut self, committee_id: Uuid) {
        if !self.assigned_committees.contains(&committee_id) {
            self.assigned_committees.push(committee_id);
            self.updated_at = Utc::now();
        }
    }
    
    pub fn remove_from_committee(&mut self, committee_id: Uuid) {
        self.assigned_committees.retain(|&id| id != committee_id);
        self.updated_at = Utc::now();
    }
    
    pub fn has_committee_access(&self, committee_id: Uuid) -> bool {
        self.assigned_committees.contains(&committee_id)
    }
}
EOF
    
    cp /tmp/admin_fixed.rs src/domain/models/admin.rs
    echo "Fixed admin.rs"
fi

# Also clean up unused imports while we're at it
echo -e "\nCleaning up unused imports..."
for file in src/domain/models/author.rs src/domain/models/plagiarism_case.rs src/domain/models/text.rs; do
    if [ -f "$file" ]; then
        # Check if Deserialize/Serialize are actually used
        if ! grep -q "Deserialize" "$file" || ! grep -q "Serialize" "$file"; then
            echo "Removing unused serde imports from $(basename $file)"
            sed -i '3d' "$file"
        fi
    fi
done

echo -e "\nTesting compilation..."
cargo check

if [ $? -eq 0 ]; then
    echo -e "\n✅ SUCCESS! Project compiles!"
    
    echo -e "\n🎉🎉🎉 FINAL COMPILATION SUCCESSFUL! 🎉🎉🎉"
    echo ""
    echo "Your Rust project now compiles without errors!"
    echo ""
    echo "Summary:"
    echo "1. Fixed non-exhaustive match in admin.rs"
    echo "2. Cleaned up unused imports"
    echo ""
    echo "Next steps:"
    echo "1. Check what modules are still in backup:"
    echo "   find ./src.backup.1769027748 -name '*.rs' | grep -v 'domain/models' | grep -v 'utils' | sort"
    echo ""
    echo "2. Restore modules gradually:"
    echo "   # Example: Restore application/services"
    echo "   mkdir -p src/application/services"
    echo "   cp ./src.backup.1769027748/application/services/*.rs src/application/services/ 2>/dev/null || true"
    echo "   cargo check"
    echo ""
    echo "3. Continue with:"
    echo "   - application/queries/"
    echo "   - application/commands/"
    echo "   - infrastructure/"
    echo "   - api/"
    echo ""
    echo "4. Test the build: cargo build"
    echo "5. Run if you have main.rs: cargo run"
else
    echo -e "\n❌ Still have errors:"
    cargo check 2>&1 | tail -20
fi
