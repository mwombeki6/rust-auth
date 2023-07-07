use sqlx::Row;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateNewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name = "Adding a new user",
skip(pool, new_user, redis_pool),
fields(
    new_user_email = %new_user.email,
    new_user_first_name = %new_user.first_name,
    new_user_last_name = %new_user.last_name
))]
#[actix_web::post("/register/")]
pub async fn register_user(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    new_user: actix_web::web::Json<NewUser>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
)