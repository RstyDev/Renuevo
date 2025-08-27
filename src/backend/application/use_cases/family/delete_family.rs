use crate::{
    backend::domain::{
        repositories::FamilyRepository, services::FamilyService,
    },
    error::AppRes,
};

pub struct DeleteFamilyUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> DeleteFamilyUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        DeleteFamilyUseCase { family_service }
    }

    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.family_service.delete(id).await
    }
}
