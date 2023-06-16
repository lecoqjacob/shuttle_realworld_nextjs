use crate::error::AppResult;
use crate::models::user::User;
use crate::utils::{authenticate, Auth};
use crate::AppState;
use actix::Message;
use actix_web::web::{self};
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use futures::FutureExt;
use serde::Serialize;

// ================================== Client Messages ================================== //
#[derive(Debug, Message)]
#[rtype(result = "AppResult<ProfileResponse>")]
pub struct GetProfile {
    // auth is option in case authentication fails or isn't present
    pub auth: Option<Auth>,
    pub username: String,
}

// ================================== JSON response objects ================================== //
#[derive(Debug, Serialize)]
pub struct ProfileResponseInner {
    pub following: bool,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

impl ProfileResponse {
    pub fn new(user: User, following: bool) -> Self {
        Self {
            profile: ProfileResponseInner {
                following,
                bio: user.bio,
                image: user.image,
                username: user.username,
            },
        }
    }
}

// ================================== Handlers ================================== //

#[actix_web::get("")]
async fn get_profile(
    req: HttpRequest,
    state: web::Data<AppState>,
    user_name: web::Path<String>,
) -> AppResult<HttpResponse> {
    let db = &state.db;
    Ok(authenticate(&state, &req)
        .then(|auth| async move {
            db.send(GetProfile {
                auth: auth.ok(),
                username: user_name.to_owned(),
            })
            .await?
        })
        .map(|res| match res {
            Err(e) => e.error_response(),
            Ok(res) => HttpResponse::Ok().json(res),
        })
        .await)
}

#[actix_web::post("/follow")]
async fn follow_profile(
    req: HttpRequest,
    state: web::Data<AppState>,
    user_name: web::Path<String>,
) -> AppResult<HttpResponse> {
    let db = &state.db;
    Ok(authenticate(&state, &req)
        .then(|auth| async move {
            db.send(GetProfile {
                auth: auth.ok(),
                username: user_name.to_owned(),
            })
            .await?
        })
        .map(|res| match res {
            Err(e) => e.error_response(),
            Ok(res) => HttpResponse::Ok().json(res),
        })
        .await)
}

#[actix_web::delete("/follow")]
async fn unfollow_profile(
    req: HttpRequest,
    state: web::Data<AppState>,
    user_name: web::Path<String>,
) -> AppResult<HttpResponse> {
    let db = &state.db;
    Ok(authenticate(&state, &req)
        .then(|auth| async move {
            db.send(GetProfile {
                auth: auth.ok(),
                username: user_name.to_owned(),
            })
            .await?
        })
        .map(|res| match res {
            Err(e) => e.error_response(),
            Ok(res) => HttpResponse::Ok().json(res),
        })
        .await)
}
