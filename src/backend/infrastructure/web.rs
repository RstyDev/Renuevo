use crate::backend::infrastructure::db::connection::establish_connection;
use crate::backend::infrastructure::repositories::surreal_family_repository::SurrealFamilyRepository;
use crate::backend::{
    infrastructure::repositories::surreal_user_repository::SurrealUserRepository,
    presentation::routes::routes::root_routes,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;
use std::env::VarError;
use surrealdb::opt::IntoQuery;

pub async fn run() -> std::io::Result<()> {
    println!("{:#?}", dotenv().ok());
    let repo = SurrealUserRepository::new().await;
    let family_repo = SurrealFamilyRepository::new().await;
    let app_data = Data::new(repo);
    let db = establish_connection().await;
    println!("Starting...");

    //App::new()
    //         .app_data(Data::new(db.to_owned()))
    //         .app_data(Data::new(env_map.to_owned()))
    //         .app_data(Config::default().realm("jwt"))
    //         .wrap(cors)
    //         .service(login)
    //         .service(refresh_token)
    //         .service(
    //             scope("")
    //                 .wrap(cors2)
    //                 .wrap(auth)
    //             //.route("/users", web::get().to(all_users))
    //             .service(all_users)
    //             .service(user_by_id)
    //             .service(new_user)
    //             .service(delete_user)
    //             .service(update_user)
    //         )

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&env::var("ORIGIN").unwrap());
        let cors = match &env::var("ORIGIN_SECOND"){
            Ok(var) => cors.allowed_origin(var).allow_any_method().allow_any_header().max_age(None),
            Err(_) => cors.allow_any_method().allow_any_header().max_age(None),
        };


        App::new()
            .app_data(app_data.to_owned())
            .app_data(Data::new(family_repo.to_owned()))
            .app_data(Data::new(db.to_owned()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(|config| root_routes(config))
    })
    .bind((env::var("HOST").expect("HOST not set").as_str(), 8088))?;
    println!("Running!");
    app.run().await
}
