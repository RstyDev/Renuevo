use crate::backend::presentation::handlers::family_handlers::{
    all_families, delete_family, family_by_id, register_family, update_family,
};
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
    let (auth, auth2) = (
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
        HttpAuthentication::with_fn(move |a, b: Option<BearerAuth>| validator(a, b)),
    );
    let (cors, cors2) = (
        Cors::default().allowed_origin(&env::var("ORIGIN").unwrap()),
        Cors::default().allowed_origin(&env::var("ORIGIN").unwrap()),
    );
    let (cors, cors2) = match &env::var("ORIGIN_SECOND") {
        Ok(var) => (
            cors.allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
            cors2
                .allowed_origin(var)
                .allow_any_method()
                .allow_any_header()
                .max_age(None),
        ),
        Err(_) => (
            cors.allow_any_method().allow_any_header().max_age(None),
            cors2.allow_any_method().allow_any_header().max_age(None),
        ),
    };

    config
        .service(login)
        .service(refresh_token)
        .service(
            web::scope("/api/v1/users")
                .wrap(cors)
                .wrap(auth)
                .service(register_user_handler)
                .service(all_users)
                .service(user_by_id)
                .service(delete_user)
                .service(update_user),
        )
        .service(
            web::scope("/api/v1/families")
                .wrap(cors2)
                .wrap(auth2)
                .service(all_families)
                .service(delete_family)
                .service(family_by_id)
                .service(register_family)
                .service(update_family),
            /*
            delete_family
            get_all_families
            get_family_by_id
            save_family
            update_family
            */
        );
}
