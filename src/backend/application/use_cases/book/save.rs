use crate::{
    error::AppRes,
    {
        backend::domain::{repositories::BookRepository, services::BookService},
        entities::Libro,
    },
};


pub struct SaveBookUseCase<T: BookRepository> {
    book_service: BookService<T>,
}

impl<T: BookRepository> SaveBookUseCase<T> {
    pub fn new(book_repo: T) -> Self {
        let book_service = BookService::new(book_repo);
        SaveBookUseCase { book_service }
    }

    pub async fn execute(&self, new_user: Libro) -> AppRes<()> {
        self.book_service.save(new_user).await
    }
}
