use diesel::pg::Pg;

use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use diesel::{
    prelude::*,
    r2d2::ConnectionManager,
};


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_dbpool() -> DbPool {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    // env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    pool
}



use diesel::prelude::*;
use diesel::query_builder::*;


pub trait OffsetLimit: Sized {
    fn offset_and_limit(self, offset: i64, limit: i64) -> OffsetLimited<Self>;
}

impl<T> OffsetLimit for T {
    fn offset_and_limit(self, offset: i64, limit: i64) -> OffsetLimited<Self> {
        OffsetLimited {
            query: self,
            limit,
            offset,
        }
    }
}

#[derive(Debug, Clone, Copy, QueryId)]
pub struct OffsetLimited<T> {
    query: T,
    offset: i64,
    limit: i64,
}

impl<T> OffsetLimited<T> {
    pub fn load_and_count<'query,U>(self, conn: &mut PgConnection) -> QueryResult<(Vec<U>, i64)>
        where
            Self: LoadQuery<'query,PgConnection, (U, i64)>,
    {
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok((records, total))
    }
}

impl<T: Query> Query for OffsetLimited<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for OffsetLimited<T> {}

impl<T> QueryFragment<Pg> for OffsetLimited<T>
    where
        T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.limit)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
