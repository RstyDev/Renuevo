use crate::backend::application::use_cases::family::delete_family::DeleteFamilyUseCase;
use crate::backend::application::use_cases::family::get_all_families::GetAllFamiliesUseCase;
use crate::backend::application::use_cases::family::get_family_by_id::GetFamilyByIdUseCase;
use crate::backend::application::use_cases::family::save_family::RegisterFamilyUseCase;
use crate::backend::application::use_cases::family::update_family::UpdateFamilyUseCase;
use crate::backend::infrastructure::repositories::surreal_family_repository::SurrealFamilyRepository;
use crate::entities::Familia;
use actix_web::web::Json;
use actix_web::{
    delete, get, post, put,
    web::{Data, Path},
    HttpResponse, Responder,
};

#[get("/")]
pub async fn all_families(repo: Data<SurrealFamilyRepository>) -> impl Responder {
    match GetAllFamiliesUseCase::new(repo.into_inner())
        .get_all()
        .await
    {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            eprintln!("Error getting all familys: {:#?}", e);
            HttpResponse::InternalServerError().json(e.to_string())
        }
    }
}

#[get("/{id}")]
pub async fn family_by_id(repo: Data<SurrealFamilyRepository>, id: Path<String>) -> impl Responder {
    match GetFamilyByIdUseCase::new(repo.into_inner())
        .get_by_id(&id.into_inner())
        .await
    {
        Ok(res) => match res {
            None => HttpResponse::NotFound().json("Family not found"),
            Some(family) => HttpResponse::Ok().json(family),
        },
        Err(e) => {
            eprintln!("Error finding family {:#?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}
#[delete("/{id}")]
pub async fn delete_family(
    repo: Data<SurrealFamilyRepository>,
    id: Path<String>,
) -> impl Responder {
    match DeleteFamilyUseCase::new(repo.into_inner())
        .delete(&id.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error while deleting family: {:?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}
#[put("/{id}")]
pub async fn update_family(
    repo: Data<SurrealFamilyRepository>,
    input: Json<Familia>,
    id: Path<String>,
) -> impl Responder {
    let repo = repo.into_inner();
    match UpdateFamilyUseCase::new(repo.clone())
        .update(input.into_inner())
        .await
    {
        Ok(family) => {
            println!("family {:#?}", family);
            HttpResponse::Ok().json(family)
        }
        Err(e) => {
            println!("Error while updating family: {:#?}", e);
            HttpResponse::InternalServerError().json(e)
        }
    }
}

#[post("/")]
pub async fn register_family(
    repo: Data<SurrealFamilyRepository>,
    input: Json<Familia>,
) -> HttpResponse {
    match RegisterFamilyUseCase::new(repo.into_inner())
        .execute(input.into_inner())
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("Error registering family: {:?}", e);
            HttpResponse::InternalServerError().body("Please try again")
        }
    }
}
