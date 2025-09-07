use crate::backend::application::use_cases::{GetAllBooksUseCase, SaveBookUseCase, DeleteBookUseCase, UpdateBookUseCase, GetBookByIdUseCase};
use actix_web::web::Json;
use actix_web::{
    delete, get, post, put,
    web::{Data, Path},
    HttpResponse, Responder,
};
use crate::backend::infrastructure::repositories::SurrealBookRepository;
use crate::entities::Libro;

#[get("/")]
pub async fn get_all_books(repo: Data<SurrealBookRepository>)->impl Responder{
    match GetAllBooksUseCase::new(repo.into_inner()).get_all().await{
        Ok(data) => {
            HttpResponse::Ok().json(data)
        }
        Err(e) => e.to_response(),
    }
}

#[get("/{id}")]
pub async fn get_book_by_id(repo: Data<SurrealBookRepository>, id: Path<String>) -> impl Responder {
    match GetBookByIdUseCase::new(repo.into_inner()).get_by_id(id.into_inner().as_str()).await {
        Ok(a) => match a{
            Some(libro) => HttpResponse::Ok().json(libro),
            None => HttpResponse::NotFound().finish(),
        },
        Err(e) => e.to_response(),
    }
}

#[post("/")]
pub async fn register_book(
    repo: Data<SurrealBookRepository>,
    input: Json<Libro>,
) -> HttpResponse {
    match SaveBookUseCase::new(repo.into_inner()).execute(input.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => e.to_response(),
    }
}

#[delete("/{id}")]
pub async fn delete_book(
    repo: Data<SurrealBookRepository>,
    id: Path<String>,
) -> impl Responder {
    match DeleteBookUseCase::new(repo.into_inner()).delete(id.into_inner().as_str()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.to_response()
    }
}


#[put("/")]
pub async fn update_book(
    repo: Data<SurrealBookRepository>,
    input: Json<Libro>,
) -> impl Responder {
    match UpdateBookUseCase::new(repo.into_inner()).update(input.into_inner()).await {
        Ok(a) => HttpResponse::Ok().json(a),
        Err(e) => e.to_response(),
    }
}