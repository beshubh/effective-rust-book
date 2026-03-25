mod errors {
    use std::collections::HashMap;

    #[derive(Debug, thiserror::Error)]
    enum AppError {
        #[error("{entity} with id {id} not found")]
        NotFound { entity: String, id: i64 },

        #[error("Validation error on {field}: {message}")]
        ValidationError { field: String, message: String },

        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),

        #[error("JSON Error: {0}")]
        Json(#[from] serde_json::Error),
    }

    #[derive(Debug, Clone)]
    struct User {
        name: String,
    }
    // Usage:
    fn find_user(user_id: i64) -> Result<User, AppError> {
        users.get(&user_id).cloned().ok_or(AppError::NotFound {
            entity: "User".to_string(),
            id: user_id,
        })
    }

    // The #[from] attribute means ? auto-converts io::Error → AppError::Io
    fn load_users(path: &str) -> Result<Vec<User>, AppError> {
        let data = fs::read_to_string(path)?; // io::Error → AppError::Io automatically
        let users: Vec<User> = serde_json::from_str(&data)?; // → AppError::Json
        Ok(users)
    }
}
