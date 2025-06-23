use crate::backend::infrastructure::db::connection::establish_connection;
use crate::backend::infrastructure::repositories::surreal_family_repository::SurrealFamilyRepository;
use crate::backend::{
    infrastructure::repositories::surreal_user_repository::SurrealUserRepository,
    presentation::routes::routes::root_routes,
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use surrealdb::opt::IntoQuery;

pub async fn run() -> std::io::Result<()> {
    println!("{:#?}", dotenv().ok());
    let env_map = env::vars().collect::<HashMap<String, String>>();
    let repo = SurrealUserRepository::new(env_map.clone()).await;
    let family_repo = SurrealFamilyRepository::new(env_map.clone()).await;
    let app_data = web::Data::new(repo);
    let db = establish_connection(env_map.clone()).await;
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
    let map = env_map.clone();
    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(map.get("ORIGIN").unwrap())
            .allowed_origin(map.get("ORIGIN_SECOND").unwrap())
            .allow_any_method()
            .allow_any_header()
            .max_age(None);

        let env_map = map.clone();
        App::new()
            .app_data(app_data.to_owned())
            .app_data(Data::new(family_repo.to_owned()))
            .app_data(Data::new(map.to_owned()))
            .app_data(Data::new(db.to_owned()))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(|config| root_routes(config, env_map))
    })
    .bind((env_map.get("HOST").expect("HOST not set").as_str(), 8088))?
    .run();
    println!("Running!");
    app.await
}
