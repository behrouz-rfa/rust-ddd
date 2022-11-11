use crate::domain::article::entity::Article;
use diesel::result::Error;
pub struct CreateArticleDto {
  pub  author: i32,
    pub title: String,
    pub  description: String,
    pub  body: String,
    pub  tag_list: Vec<String>,
    pub slug:String,
}

impl TryFrom<CreateArticleDto> for Article {
    type Error = Error;

    fn try_from(value: CreateArticleDto) -> Result<Self, Self::Error> {
        Ok(Article{
            id: 0,
            slug:value.slug,
            title: value.title,
            description:value.description,
            body: value.body,
            author: value.author,
            tag_list: value.tag_list,
            created_at: Default::default(),
            updated_at: Default::default(),
            favorites_count: 0
        })
    }
}