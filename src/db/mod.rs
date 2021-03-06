use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

pub(crate) type ConnectionPool = Arc<Pool<ConnectionManager<PgConnection>>>;

pub(crate) fn create_database_pool(url: &str, size: u32) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(size)
        .build(manager)
        .expect("Error creating a database pool");

    Arc::new(pool)
}

pub mod sql {
    pub use svc_agent::sql::{Account_id, Agent_id};
}

pub(crate) mod janus_backend;
pub(crate) mod janus_rtc_stream;
pub(crate) mod recording;
pub(crate) mod room;
pub(crate) mod rtc;
