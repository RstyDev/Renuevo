use crate::{
    error::AppRes,
    {
        backend::domain::{
            repositories::FamilyRepository,
            services::FamilyService,
        },
        entities::Familia,
    },
};

pub struct RegisterFamilyUseCase<T: FamilyRepository> {
    family_service: FamilyService<T>,
}

impl<T: FamilyRepository> RegisterFamilyUseCase<T> {
    pub fn new(family_repo: T) -> Self {
        let family_service = FamilyService::new(family_repo);
        RegisterFamilyUseCase { family_service }
    }

    pub async fn execute(&self, new_family: Familia) -> AppRes<()> {
        self.family_service.register_family(new_family).await
    }
}
