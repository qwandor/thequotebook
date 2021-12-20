use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quotes_count: i64,
}
