use actix_web::{
    body::MessageBody,
    cookie::Cookie,
    dev::{ServiceRequest, ServiceResponse},
    error,
    http::header::{HeaderValue, SET_COOKIE},
    middleware::Next,
    HttpMessage,
};
use serde_json::json;

use crate::utils::{decode_token, generate_access_token};

pub async fn jwt_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> actix_web::Result<ServiceResponse<impl MessageBody>> {
    match req.cookie("access_token") {
        Some(token) => {
            let access_token = token.value().to_string();
            match decode_token(&access_token, "secret") {
                Ok(claims) => {
                    // Insert the claims into the request extensions
                    req.extensions_mut().insert(claims.claims);
                    return next.call(req).await;
                }
                Err(error) => match error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        match req.cookie("refresh_token") {
                            Some(token) => {
                                let refresh_token = token.value().to_string();
                                match decode_token(&refresh_token, "secret") {
                                    Ok(claims) => {
                                        // Generate a new access token
                                        let new_access_token = generate_access_token(
                                            &claims.claims.sub,
                                            "secret",
                                        )
                                        .map_err(|_| {
                                            error::ErrorInternalServerError(json!({
                                                "error": "Error generating new access token!"
                                            }))
                                        })?;

                                        // Create a new cookie for the access token
                                        let cookie =
                                            Cookie::build("access_token", new_access_token.clone())
                                                .http_only(true)
                                                .path("/")
                                                .finish();

                                        // Add the claims to the request
                                        req.extensions_mut().insert(claims.claims);

                                        // Call the next service
                                        let mut response = next.call(req).await?;

                                        // Add the cookie to the response headers
                                        response.response_mut().headers_mut().insert(
                                            SET_COOKIE,
                                            HeaderValue::from_str(&cookie.to_string()).map_err(
                                                |_| {
                                                    error::ErrorInternalServerError(
                                                        "Invalid cookie",
                                                    )
                                                },
                                            )?,
                                        );

                                        return Ok(response);
                                    }
                                    Err(e) => {
                                        return Err(error::ErrorUnauthorized(
                                            json!({"error": format!("Invalid or expired refresh token: {}", e)}),
                                        ));
                                    }
                                }
                            }
                            None => {
                                return Err(error::ErrorUnauthorized(
                                    json!({"error": "Missing refresh token"}),
                                ));
                            }
                        }
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                        return Err(error::ErrorUnauthorized(
                            json!({"error": "Invalid token. Please try again!"}),
                        ));
                    }
                    _ => {
                        return Err(error::ErrorUnauthorized(json!({"error": "Token error!"})));
                    }
                },
            }
        }
        None => {
            return Err(error::ErrorUnauthorized("Missing access token"));
        }
    }
}
