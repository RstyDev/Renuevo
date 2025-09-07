use crate::{backend::domain::repositories::FamilyRepository, entities::Familia, error::AppRes};

#[derive(Clone)]
pub struct FamilyService<T: FamilyRepository> {
    family_repo: T,
}

impl<T: FamilyRepository> FamilyService<T> {
    pub fn new(family_repo: T) -> Self {
        Self { family_repo }
    }
    pub async fn register_family(&self, familia: Familia) -> AppRes<()> {
        self.family_repo.save(&familia).await
    }
    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        self.family_repo.get_by_id(id).await
    }
    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.family_repo.delete(id).await
    }
    pub async fn get_all(&self) -> AppRes<Vec<Familia>> {
        self.family_repo.get_all().await
    }
    pub async fn update(&self, familia: Familia) -> AppRes<()> {
        self.family_repo.update(familia).await
    }
}
