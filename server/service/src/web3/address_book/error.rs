// Error types for Address Book

#[derive(Debug)]
pub struct ServiceError {
    pub code: i32,
    pub message: String,
}

impl ServiceError {
    pub fn new(msg: &str) -> Self {
        Self { code: 500, message: msg.to_string() }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceError {}

pub type ServiceResult<T> = Result<T, ServiceError>;
