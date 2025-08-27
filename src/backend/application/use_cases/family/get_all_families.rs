use crate::{
    backend::domain::{
        repositories::FamilyRepository, services::FamilyService,
    },
    entities::Familia,
    error::AppRes,
};

pub struct GetAllFamiliesUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> GetAllFamiliesUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        GetAllFamiliesUseCase { family_service }
    }

    pub async fn get_all(&self) -> AppRes<Vec<Familia>> {
        self.family_service.get_all().await
    }
}
