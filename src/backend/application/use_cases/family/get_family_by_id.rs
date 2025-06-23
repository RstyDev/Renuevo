use crate::{
    backend::domain::{
        repositories::family_repository::FamilyRepository, services::family_service::FamilyService,
    },
    entities::Familia,
    error::AppRes,
};

pub struct GetFamilyByIdUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> GetFamilyByIdUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        GetFamilyByIdUseCase { family_service }
    }

    pub async fn get_by_id(&self, id: &str) -> AppRes<Option<Familia>> {
        self.family_service.get_by_id(id).await
    }
}
