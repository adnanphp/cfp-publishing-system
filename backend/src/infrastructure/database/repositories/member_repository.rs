use sqlx::PgPool;

#[derive(Clone)]
pub struct MemberRepository {
    pool: PgPool,
}

impl MemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    // Add methods here later
}
