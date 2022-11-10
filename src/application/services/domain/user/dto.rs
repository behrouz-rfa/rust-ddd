use diesel::result::Error;
use crate::domain::user::entity::User;

pub struct NewUserDto {
    pub(crate) email: String,
    pub username: String,
    pub password: String,

}

impl TryFrom<NewUserDto> for User {
    type Error = Error;
    fn try_from(dto: NewUserDto) -> Result<Self, Self::Error> {
        Ok(User {
            id: 0,
            username: dto.username,
            hash: dto.password,
            email: dto.email,
            bio: None,
            image: None,
        })
    }
}