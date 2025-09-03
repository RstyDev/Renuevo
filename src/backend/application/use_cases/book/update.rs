use crate::{
    backend::domain::{repositories::BookRepository, services::BookService},
    entities::Libro,
    error::AppRes,
};

pub struct UpdateBookUseCase<T: BookRepository> {
    book_service: BookService<T>,
}

impl<T: BookRepository> UpdateBookUseCase<T> {
    pub fn new(book_repo: T) -> Self {
        let book_service = BookService::new(book_repo);
        UpdateBookUseCase { book_service }
    }

    pub async fn update(&self, libro: Libro) -> AppRes<Libro> {
        self.book_service.update(libro).await
    }
}
