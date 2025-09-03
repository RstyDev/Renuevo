#![allow(async_fn_in_trait)]

use crate::entities::Libro;
use crate::error::AppRes;

pub trait BookRepository {
    async fn save(&self, book: Libro) -> AppRes<()>;
    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Libro>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Libro>>;
    async fn get_by_user(&self, user: &str) -> AppRes<Vec<Libro>>;
    async fn update(&self, persona: Libro) -> AppRes<Libro>;
}