use crate::{
    error::AppRes,
    {
        backend::domain::{
            repositories::user_repository::UserRepository, services::user_service::UserService,
        },
        entities::Persona,
    },
};

pub struct RegisterUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> RegisterUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        RegisterUserUseCase { user_service }
    }

    pub async fn execute(&self, new_user: Persona) -> AppRes<()> {
        self.user_service.register_user(new_user).await
    }
}
