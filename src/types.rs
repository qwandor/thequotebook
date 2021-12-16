pub struct Flash {
    pub notice: Option<String>,
    pub error: Option<String>,
}

pub struct Context {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quote_count: u32,
}
