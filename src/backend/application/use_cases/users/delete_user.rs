use crate::{
    backend::domain::{
        repositories::UserRepository, services::UserService,
    },
    error::AppRes,
};

pub struct DeleteUserUseCase<T: UserRepository> {
    user_service: UserService<T>,
}

impl<T: UserRepository> DeleteUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        DeleteUserUseCase { user_service }
    }

    pub async fn delete(&self, id: &str) -> AppRes<()> {
        self.user_service.delete(id).await
    }
}
