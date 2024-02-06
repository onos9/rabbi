use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn require_auth(cookies: Cookies, req: Request, next: Next) -> Result<Response> {
    println!("->> {:<12} - require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let (user_id, _exp, _sig) = auth_token
        .ok_or(Error::AuthFailedNotAuthTokenCookie)
        .and_then(parse_token)?;

    // TODO: Implement token component validation

    Ok(next.run(req).await)
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sig) = regex_captures!(r"^user-(\d+)\.(.+)\.(.+)$", &token)
        .ok_or(Error::AuthFailedTokenWrongFormat)?;

    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailedTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sig.to_string()))
}
