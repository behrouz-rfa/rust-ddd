use crate::domain::article::entity::{Article, FeedArticles, FindArticles};
use diesel::result::Error;
use crate::infrastructure::domain::article::dto::UpdateArticleData;

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



pub struct FindArticlesDto {
    pub tag: Option<String>,
    pub author: Option<String>,
    /// favorited by user
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}



pub struct FeedArticlesDto {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub struct UpdateArticleDataDto {
  pub title: Option<String>,
  pub description: Option<String>,
  pub body: Option<String>,
  pub slug: Option<String>,
  pub tag_list: Vec<String>,
}

impl TryFrom<UpdateArticleDataDto> for UpdateArticleData {
    type Error = Error;
    fn try_from(value: UpdateArticleDataDto)-> Result<Self,Self::Error> {
        Ok (
            UpdateArticleData {
                title: value.title,
                description: value.description,
                body: value.body,
                slug: value.slug,
                tag_list: value.tag_list
            }
        )
    }
}
impl TryFrom<FeedArticlesDto> for FeedArticles {
    type Error = Error;
    fn try_from(value: FeedArticlesDto)-> Result<Self,Self::Error> {
        Ok(
            FeedArticles{
                limit: value.limit,
                offset: value.offset
            }
        )
    }
}


impl TryFrom<FindArticlesDto> for FindArticles {
    type Error = Error;

    fn try_from(value: FindArticlesDto) -> Result<Self, Self::Error> {
       Ok(
           FindArticles{
               tag: value.tag,
               author: value.author,
               favorited: value.favorited,
               limit: value.limit,
               offset: value.offset
           }
       )
    }
}