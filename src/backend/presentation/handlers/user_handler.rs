use crate::{
    backend::{
        application::use_cases::users::{
            delete_user::DeleteUserUseCase, get_all_users::GetAllUsersUseCase,
            get_user_by_id::GetUserByIdUseCase, get_user_by_mail::GetUserByEmailUseCase,
            register_user::RegisterUserUseCase, update_user::UpdateUserUseCase,
        },
        infrastructure::repositories::surreal_user_repository::SurrealUserRepository,
    },
    entities::Persona,
};
use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};

#[get("/")]
pub async fn all_users(repo: Data<SurrealUserRepository>) -> impl Responder {
    match GetAllUsersUseCase::new(repo.into_inner()).get_all().await {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error getting all users: {:#?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}
#[get("/{id}")]
pub async fn user_by_id(repo: Data<SurrealUserRepository>, id: Path<String>) -> impl Responder {
    match GetUserByIdUseCase::new(repo.into_inner())
        .get_by_id(&id.into_inner())
        .await
    {
        Ok(res) => match res {
            None => HttpResponse::NotFound().json("User not found"),
            Some(persona) => HttpResponse::Ok().json(persona),
        },
        Err(e) => {
            eprintln!("Error finding user {:#?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

#[delete("/{id}")]
pub async fn delete_user(repo: Data<SurrealUserRepository>, id: Path<String>) -> impl Responder {
    match DeleteUserUseCase::new(repo.into_inner())
        .delete(&id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error while deleting user: {:?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}
#[patch("/users")]
pub async fn update_user(
    repo: Data<SurrealUserRepository>,
    input: Json<Persona>,
) -> impl Responder {
    match UpdateUserUseCase::new(repo.into_inner())
        .update(input.into_inner())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Error while updating user: {:#?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}
#[post("/")]
pub async fn register_user_handler(
    repo: Data<SurrealUserRepository>,
    input: Json<Persona>,
) -> HttpResponse {
    match RegisterUserUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error registering user: {:?}", e);
            HttpResponse::InternalServerError().body("Please try again")
        }
    }
}

#[get("/email/{email}")]
pub async fn get_by_email(repo: Data<SurrealUserRepository>, email: Path<String>) -> HttpResponse {
    match GetUserByEmailUseCase::new(repo.into_inner())
        .get_by_email(&email.into_inner())
        .await
    {
        None => {
            eprintln!("User not found");
            HttpResponse::NotFound().finish()
        }
        Some(u) => HttpResponse::Ok().json(u),
    }
}
