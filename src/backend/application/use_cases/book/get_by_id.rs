use crate::{
    backend::domain::{repositories::BookRepository, services::BookService},
    entities::Libro,
    error::AppRes,
};

pub struct GetBookByIdUseCase<T: BookRepository> {
    book_service: BookService<T>,
}

impl<T: BookRepository> GetBookByIdUseCase<T> {
    pub fn new(book_repo: T) -> Self {
        let book_service = BookService::new(book_repo);
        GetBookByIdUseCase { book_service }
    }

    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Libro>> {
        self.book_service.get_by_id(id).await
    }

}
