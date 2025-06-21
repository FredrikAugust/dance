use axum::{http::StatusCode, response::IntoResponse};

pub struct ApplicationError(anyhow::Error);

impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for ApplicationError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
