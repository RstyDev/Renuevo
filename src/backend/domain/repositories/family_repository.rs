#![allow(async_fn_in_trait)]
use crate::{entities::Familia, error::AppRes};

pub trait FamilyRepository {
    async fn save(&self, user: &Familia) -> AppRes<()>;

    async fn delete(&self, id: &str) -> AppRes<()>;
    async fn get_all(&self) -> AppRes<Vec<Familia>>;
    async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>>;
    async fn update(&self, persona: Familia, id: String) -> AppRes<()>;
}
