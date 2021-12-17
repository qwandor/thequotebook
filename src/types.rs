use sqlx::FromRow;

pub struct Flash {
    pub notice: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quote_count: u32,
}

#[derive(Clone, Debug, FromRow)]
pub struct Quote {
    pub id: i32,
    pub quote_text: String,
    pub context_id: i32,
    pub quoter_id: i32,
    pub quotee_id: i32,
    pub hidden: bool,
}

impl Quote {
    pub fn comments_count(&self) -> u32 {
        // TODO
        0
    }

    pub fn quotee(&self) -> User {
        User {
            id: 0,
            email: "user@domain.com".to_string(),
            username: "quotee".to_string(),
            fullname: "Full Quotee".to_string(),
        }
    }

    pub fn quoter(&self) -> User {
        User {
            id: 0,
            email: "user@domain.com".to_string(),
            username: "quoter".to_string(),
            fullname: "Full Quoter".to_string(),
        }
    }

    pub fn context(&self) -> Context {
        Context {
            id: 0,
            name: "Context".to_string(),
            description: "Description".to_string(),
            quote_count: 0,
        }
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub fullname: String,
}
