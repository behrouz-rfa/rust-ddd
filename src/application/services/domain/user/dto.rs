use diesel::result::Error;
use crate::domain::user::entity::{UpdateUserData, User};

pub struct NewUserDto {
    pub(crate) email: String,
    pub username: String,
    pub password: String,

}


pub struct UpdateUserDto {
    pub  username: Option<String>,
    pub  email: Option<String>,
    pub  bio: Option<String>,
    pub  image: Option<String>,

    // hack to skip the field
    pub password: Option<String>,
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


impl TryFrom<UpdateUserDto> for UpdateUserData {
    type Error = Error;
    fn try_from(dto: UpdateUserDto) -> Result<Self, Self::Error> {
        Ok(UpdateUserData {
            username: dto.username,
            email: dto.email,
            bio: dto.bio,
            image: dto.image,
            password: None
        })
    }
}


