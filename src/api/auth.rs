use crate::api::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData {
    pub login: String,
    pub password: String,
}

pub async fn login(
    auth: web::Json<AuthData>,
    id: ActixIdentity,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    let auth = auth.into_inner();
    let user: user::Model = query(&auth, state).await?;
    let user_string = serde_json::to_string(&user).unwrap();
    println!("{:?}", user_string);
    id.remember(user_string);
    Ok(HttpResponse::Ok().finish())
}

pub async fn logout(id: ActixIdentity) -> Result<HttpResponse, ServiceError> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

pub async fn get(id: ActixIdentity) -> HttpResponse {
    // id.identity()
    // let id: &str = &id.identity().unwrap()[..];
    // let id = serde_json::from_str(id).unwrap();
    HttpResponse::Ok().json(id.identity())
}

async fn query(auth: &AuthData, state: web::Data<AppState>) -> Result<user::Model, ServiceError> {
    let user = User::find()
        .filter(
            user::Column::Email
                .eq(auth.login.clone())
                .or(user::Column::Username.eq(auth.login.clone())),
        )
        .one(&state.conn)
        .await?;
    if let Some(user) = user {
        if let Ok(matching) = crate::auth::verify(&user.hash, &auth.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
