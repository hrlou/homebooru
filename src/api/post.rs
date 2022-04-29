use crate::api::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct PostOutput {
    post: post::Model,
    tags: Vec<tag::Model>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagInput {
    kind: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PostInput {
    title: Option<String>,
    source: Option<String>,
}

pub async fn get(path: web::Path<i32>, user: user::Model, state: web::Data<AppState>) -> Result<HttpResponse, ServiceError> {
    let post = path.into_inner();
    let (post, tags) = Post::get(&state.conn, post).await?;
    let output = PostOutput { post, tags };
    Ok(HttpResponse::Ok().json(output))
}

// pub async fn post(id: ActixIdentity, path: web::Path<i32>, state: web::Data<AppState>)