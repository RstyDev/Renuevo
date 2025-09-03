use crate::{
    backend::domain::{repositories::BookRepository, services::BookService},
    entities::Libro,
    error::AppRes,
};

pub struct GetAllBooksUseCase<T: BookRepository> {
    book_service: BookService<T>,
}

impl<T: BookRepository> GetAllBooksUseCase<T> {
    pub fn new(book_repo: T) -> Self {
        let book_service = BookService::new(book_repo);
        GetAllBooksUseCase { book_service }
    }

    pub async fn get_all(&self) -> AppRes<Vec<Libro>> {
        self.book_service.get_all().await
    }
}
