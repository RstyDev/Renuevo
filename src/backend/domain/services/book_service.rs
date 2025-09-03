use crate::backend::domain::repositories::BookRepository;
use crate::entities::Libro;
use crate::error::AppRes;

#[derive(Clone)]
pub struct BookService<T: BookRepository> {
    book_repo: T,
}

impl <T: BookRepository> BookService<T> {
    pub fn new(book_repo: T) -> Self {
        Self{book_repo}
    }
    pub async fn get_all(&self) -> AppRes<Vec<Libro>> {
        self.book_repo.get_all().await
    }
    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Libro>> {
        self.book_repo.get_by_id(id).await
    }
    pub async fn get_by_user(&self, user: &str) -> AppRes<Vec<Libro>> {
        self.book_repo.get_by_user(user).await
    }
    pub async fn save(&self, book: Libro) -> AppRes<()> {
        self.book_repo.save(book).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.book_repo.delete(id).await
    }
    pub async fn update(&self, book: Libro) -> AppRes<Libro> {
        self.book_repo.update(book).await
    }
}