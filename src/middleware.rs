use std::{
    fs::read_to_string,
    future::{ready, Ready},
    path::PathBuf,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, FromRequest, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use paperclip::actix::Apiv2Security;
use serde::{Deserialize, Serialize};

use crate::error::Error as AppError;

#[derive(Clone, Debug, Deserialize, Serialize, Apiv2Security)]
#[openapi(
    apiKey,
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
pub struct UserClaims {
    pub sub: String,
    pub exp: usize,
    pub scopes: Vec<String>,
}

impl FromRequest for UserClaims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let ext = req.extensions();
        let user_claims = match ext.get::<UserClaims>() {
            None => return ready(Err(AppError::Unauthorized.into())),
            Some(user_claims) => user_claims,
        };

        ready(Ok(user_claims.clone()))
    }
}

pub struct JwtAuth {
    pub_key: String,
}

impl JwtAuth {
    pub fn new(pub_key_path: PathBuf) -> Self {
        let pub_key = read_to_string(pub_key_path).expect("could not open file for public key");

        JwtAuth { pub_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service,
            pub_key: self.pub_key.clone(),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
    pub_key: String,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = match req.headers().get("Authorization") {
            None => {
                tracing::info!("No auth header");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
            Some(auth_header) => auth_header,
        };

        let bearer = match auth_header.to_str() {
            Err(err) => {
                tracing::error!("Err getting bearer token: {err}");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
            Ok(bearer) => bearer,
        };

        let token = bearer.split("Bearer").collect::<Vec<&str>>()[1].trim();
        let token_data = match decode::<UserClaims>(
            token,
            &DecodingKey::from_rsa_pem(self.pub_key.as_bytes()).unwrap(),
            &Validation::new(Algorithm::RS256),
        ) {
            Err(err) => {
                tracing::error!("Err validating bearer token: {err}");
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) });
            }
            Ok(token_data) => token_data,
        };

        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);
        Box::pin(async move { Ok(fut.await?) })
    }
}
