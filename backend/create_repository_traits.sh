#!/bin/bash

# Create a simple trait definition for repositories
cat > src/domain/repositories/mod.rs << 'EOF'
pub mod traits;

pub use traits::*;
