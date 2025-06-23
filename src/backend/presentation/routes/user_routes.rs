use crate::backend::presentation::handlers::user_handler::{
    all_users, delete_user, get_by_email, register_user_handler, update_user, user_by_id,
};
use actix_web::web;

pub fn user_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/users")
            .service(register_user_handler)
            .service(get_by_email)
            .service(all_users)
            .service(user_by_id)
            .service(delete_user)
            .service(update_user),
    );
}
