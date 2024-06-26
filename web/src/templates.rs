use std::collections::HashMap;

use axum::{
    extract::{FromRequestParts, Query},
    response::{Html, IntoResponse, Response},
};
use maud::PreEscaped;

use crate::{err, AppState};

pub struct Flash {
    error: Option<String>,
    success: Option<String>,
}

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Flash {
    type Rejection = err::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let Query(query) =
            Query::<HashMap<String, String>>::from_request_parts(parts, _state).await?;
        let error = query.get("flash[error]").cloned();
        let success = query.get("flash[success]").cloned();

        Ok(Flash { error, success })
    }
}

pub struct Template {
    flash: Flash,
}

#[async_trait::async_trait]
impl FromRequestParts<AppState> for Template {
    type Rejection = err::Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let flash = Flash::from_request_parts(parts, state).await?;

        Ok(Template { flash })
    }
}

impl Template {
    pub fn render(self, inner: maud::Markup) -> TemplatedPage {
        TemplatedPage {
            inner,
            template: self,
        }
    }
}

pub struct TemplatedPage {
    inner: maud::Markup,
    template: Template,
}

impl TemplatedPage {
    fn html(self) -> PreEscaped<String> {
        maud::html! {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                script src="https://cdn.tailwindcss.com" {}
            }

            body {
                @if let Some(error) = &self.template.flash.error {
                    p class="color-red-500" { (error) }
                }
                @if let Some(success) = &self.template.flash.success {
                    p class="color-green-500" { (success) }
                }
                (self.inner)
            }
        }
    }
}

impl IntoResponse for TemplatedPage {
    fn into_response(self) -> Response {
        Html(self.html().into_string()).into_response()
    }
}
