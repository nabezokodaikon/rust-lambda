use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    Ok(json!({
        "message": "AWS Lambda on Rust"
    }))
}

fn main() {
    lambda!(handler)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handler_handles() {
        let request: Request = Request::default();
        let expected = json!({
            "message": "AWS Lambda on Rust"
        })
        .into_response();
        let response = handler(request, Context::default())
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
