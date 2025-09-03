use crate::{
    backend::domain::{repositories::BookRepository, services::BookService},
    error::AppRes,
};

pub struct DeleteBookUseCase<T: BookRepository> {
    book_service: BookService<T>,
}

impl<T: BookRepository> DeleteBookUseCase<T> {
    pub fn new(book_repo: T) -> Self {
        let book_service = BookService::new(book_repo);
        DeleteBookUseCase { book_service }
    }

    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.book_service.delete(id).await
    }
}
