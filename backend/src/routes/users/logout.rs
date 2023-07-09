#[tracing::instrument(name = "Logout user", skip(session))]
#[actix_web::post("/logout/")]
