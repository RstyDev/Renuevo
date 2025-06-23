use crate::{
    backend::domain::{
        repositories::family_repository::FamilyRepository, services::family_service::FamilyService,
    },
    entities::Familia,
    error::AppRes,
};

pub struct UpdateFamilyUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> UpdateFamilyUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        UpdateFamilyUseCase { family_service }
    }

    pub async fn update(&self, persona: Familia) -> AppRes<()> {
        self.family_service.update(persona).await
    }
}
