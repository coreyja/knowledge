use std::fmt::Display;

use axum::response::IntoResponse;
use tracing::error;

#[derive(Debug)]
pub struct Error {
    report: color_eyre::Report,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.report.fmt(f)
    }
}

impl<E> From<E> for Error
where
    E: Into<color_eyre::Report>,
{
    fn from(error: E) -> Self {
        Self {
            report: error.into(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        error!("Error: {:#?}", self);

        axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
