use crate::backend::presentation::handlers::root_handlers::{login, refresh_token, validator};
use crate::backend::presentation::handlers::user_handler::{
    all_users, delete_user, register_user_handler, update_user, user_by_id,
};
use actix_cors::Cors;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;

pub fn root_routes(config: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| {
        validator(a, b)
    });
    let cors = Cors::default()
        .allowed_origin(&env::var("ORIGIN").unwrap());
    let cors = match &env::var("ORIGIN_SECOND"){
        Ok(var) => cors.allowed_origin(var).allow_any_method().allow_any_header().max_age(None),
        Err(_) => cors.allow_any_method().allow_any_header().max_age(None),
    };

    config.service(login).service(refresh_token).service(
        web::scope("/api/v1/users")
            .wrap(cors)
            .wrap(auth)
            .service(register_user_handler)
            .service(all_users)
            .service(user_by_id)
            .service(delete_user)
            .service(update_user),
    );
}

//
// .service(
// scope("/api/v1")
// .wrap(cors2).wrap(auth)
// .configure(user_routes)
// )
