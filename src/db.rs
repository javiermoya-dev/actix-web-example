use diesel::{mysql::MysqlConnection};
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use actix_web::{http::StatusCode};
use r2d2::{self, PooledConnection};
use std::env;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);

        Pool::builder().build(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
}

pub fn connection() -> DbConnection { 
    return POOL.get().unwrap();
}   
