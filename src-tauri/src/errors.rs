use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
}

impl AppError {
    pub fn not_found(entity: &str, id: &str) -> Self {
        Self {
            code: "NOT_FOUND".into(),
            message: format!("{entity} {id} 不存在"),
        }
    }

    pub fn db_error(msg: &str) -> Self {
        Self {
            code: "DB_ERROR".into(),
            message: msg.into(),
        }
    }

    pub fn validation(msg: &str) -> Self {
        Self {
            code: "VALIDATION".into(),
            message: msg.into(),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        Self::db_error(&e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::db_error(&e.to_string())
    }
}