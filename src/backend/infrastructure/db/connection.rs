use std::collections::HashMap;
use std::env;
use surrealdb::{
    engine::any::{connect, Any},
    opt::auth::Root,
    Surreal,
};

pub type DBPool = Surreal<Any>;

pub async fn establish_connection(env_map: HashMap<String, String>) -> DBPool {
    let db = connect(env_map.get("DB_URL").expect("DB URL not set"))
        .await
        .expect("Failed to establish connection");
    db.use_ns("church")
        .use_db("personas")
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        .unwrap();

    // Authenticate
    db.signin(Root {
        username: env_map.get("DB_LOGIN").expect("DB LOGIN not set"),
        password: env_map.get("DB_PASSWORD").expect("DB PASSWORD not set"),
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    .unwrap();
    db
}
