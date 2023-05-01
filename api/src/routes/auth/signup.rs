use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct SignupForm {
    username: String,
    password: String,
    email: String,
}

#[post("/signup")]
async fn signup(
    form: web::Form<SignupForm>,
    identity: Identity,
) -> Result<HttpResponse, HttpResponse> {
    if let Some(identity) = identity {
        // ユーザーを作成する
        let user = User::new(1, form.username.clone(), form.password.clone(), form.email.clone());

        // Identityオブジェクトにユーザー情報を格納する
        user.to_identity(&identity);

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(HttpResponse::BadRequest().finish())
    }
}
