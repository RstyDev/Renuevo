use crate::{
    backend::{
        application::use_cases::{
            DeleteUserUseCase, GetAllUsersUseCase, GetUserByIdUseCase, RegisterUserUseCase,
            UpdateUserUseCase,
        },
        infrastructure::repositories::SurrealUserRepository,
    },
    entities::{dtos::PasswordChange, Persona},
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use crate::error::AppError;

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
#[put("/{id}/password")]
pub async fn change_password(
    repo: Data<SurrealUserRepository>,
    input: Json<PasswordChange>,
    id: Path<String>,
) -> impl Responder {
    let path_user = input.into_inner();
    let id = id.into_inner();
    if !(path_user.id.eq(&id)) {
        return HttpResponse::BadRequest().json("Id in path must be the same as user id in JSON");
    }
    let repo = repo.into_inner();
    match UpdateUserUseCase::new(repo.clone())
        .is_id_pass_correct(id.as_str(),&path_user.old_password)
        .await
    {
        Ok(res) => match res {
            true => {
                // user.set_password(Some(path_user.new_password));
                match UpdateUserUseCase::new(repo.clone())
                    .update_password(id.as_str(),&path_user.new_password)
                    .await
                {
                    Ok(_) => {
                        println!("User");
                        HttpResponse::NoContent().finish()
                    }
                    Err(e) => {
                        println!("Error while updating user: {:#?}", e);
                        HttpResponse::InternalServerError().json(e)
                    }
                }
            }
            false => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            match e {
                AppError::ValidationErr(_,e) => HttpResponse::BadRequest().json(e),
                AppError::NotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        },
    }
}
#[put("/{id}")]
pub async fn update_user(
    repo: Data<SurrealUserRepository>,
    input: Json<Persona>,
    id: Path<String>,
) -> impl Responder {
    let repo = repo.into_inner();
    match GetUserByIdUseCase::new(repo.clone())
        .get_by_id_with_password(id.into_inner().as_str())
        .await
    {
        Ok(res) => match res {
            Some(mut user) => {
                let path_user = input.into_inner();
                println!("From DB: {:#?}", user);
                user.set_apellido(path_user.apellido().to_owned());
                user.set_estado_civil(path_user.estado_civil().to_owned());
                user.set_estado(path_user.estado().to_owned());
                println!("New: {:#?}", user);
                match UpdateUserUseCase::new(repo.clone())
                    .update(user)
                    .await
                {
                    Ok(user) => {
                        println!("User {:#?}", user);
                        HttpResponse::Ok().json(user)
                    }
                    Err(e) => {
                        println!("Error while updating user: {:#?}", e);
                        HttpResponse::InternalServerError().json(e)
                    }
                }
            }
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
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
