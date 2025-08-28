use crate::{
    backend::domain::{repositories::FamilyRepository, services::FamilyService},
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

    pub async fn update(&self, persona: Familia, id: String) -> AppRes<()> {
        self.family_service.update(persona, id).await
    }
}
