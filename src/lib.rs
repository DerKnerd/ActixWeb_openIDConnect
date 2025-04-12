//! Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
//!
//! # Examples
//! Lightweight async OpenID Connect (OIDC) client and middleware for Actix-Web.
//! Support for the Authorization Code Flow
//! Documentation: https://github.com/RomainMichau/ActixWeb_openIDConnect
//!

use std::sync::Arc;

use actix_web::dev::ServiceRequest;
use actix_web::web;
use actix_web::web::ServiceConfig;

use crate::openid::OpenID;

mod openid;
pub mod openid_middleware;

#[derive(Clone)]
pub struct ActixWebOpenId {
    openid_client: Arc<OpenID>,
    should_auth: fn(&ServiceRequest) -> bool,
    use_pkce: bool,
}

impl ActixWebOpenId {
    #[allow(clippy::too_many_arguments)]
    pub async fn init(
        client_id: String,
        client_secret: Option<String>,
        redirect_url: String,
        issuer_url: String,
        should_auth: fn(&ServiceRequest) -> bool,
        post_logout_redirect_url: Option<String>,
        scopes: Vec<String>,
        additional_audiences: Vec<String>,
        use_pkce: bool,
    ) -> Self {
        Self {
            openid_client: Arc::new(
                OpenID::init(
                    client_id,
                    client_secret,
                    redirect_url,
                    issuer_url,
                    post_logout_redirect_url,
                    scopes,
                    additional_audiences,
                    use_pkce,
                )
                .await
                .unwrap(),
            ),
            should_auth,
            use_pkce,
        }
    }

    pub fn configure_open_id(&self) -> impl Fn(&mut ServiceConfig) {
        let client = self.openid_client.clone();
        move |cfg: &mut ServiceConfig| {
            cfg.service(openid_middleware::auth_endpoint)
                .service(openid_middleware::logout_endpoint)
                .app_data(web::Data::new(client.clone()));
        }
    }

    pub fn get_middleware(&self) -> openid_middleware::AuthenticateMiddlewareFactory {
        openid_middleware::AuthenticateMiddlewareFactory::new(
            self.openid_client.clone(),
            self.should_auth,
            self.use_pkce,
        )
    }
}
