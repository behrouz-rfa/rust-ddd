use crate::application::services::domain::user::dto::NewUserDto;
use crate::domain::user::entity::User;
use crate::domain::user::repository::Repository as UserRepository;
use crate::error::Result;
pub struct UserService<R>
where R:UserRepository {
    user_repository: R,
}

impl<R> UserService<R>
where R:UserRepository
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }


    pub fn create(&self, user: NewUserDto) -> Result<User> {
        let user = User::try_from(user)?;
        let user = self.user_repository.create(&user)?;
        Ok(user)

    }
    pub fn find_by(&self, dto: NewUserDto)-> Result<User> {
        let user= User::try_from(dto)?;
        let user = self.user_repository.find_by(&user)?;
        Ok(user)
    }
}

