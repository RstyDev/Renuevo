use crate::{
    backend::domain::{
        repositories::family_repository::FamilyRepository, services::family_service::FamilyService,
    },
    error::AppRes,
};

pub struct DeleteUserUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> DeleteUserUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        DeleteUserUseCase { family_service }
    }

    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.family_service.delete(id).await
    }
}
