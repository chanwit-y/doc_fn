use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .min_idle(Some(2))
            .build(manager)
            .expect("Failed to create database connection pool");

        Database { pool }
    }

    pub fn from_env() -> Self {
	let database_url = build_database_url();
	Self::new(&database_url)
    }

    pub fn get_connection(&self) -> Result<PgPooledConnection, r2d2::PoolError> {
		self.pool.get()
    }

    pub fn pool(&self) -> &PgPool {
		&self.pool
    }
}

fn build_database_url() -> String {
	let pg_vars = (
		env::var("PGHOST").ok(),
		env::var("PGUSER").ok(),
		env::var("PGPASSWORD").ok(),
		env::var("PGDATABASE").ok(),
	);

	match pg_vars {
		(Some(host), Some(user), Some(password), Some(database)) => {
			let port = env::var("PGPORT").unwrap_or_else(|_| "5432".to_string());
			format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, database)
		}
		_ => env::var("DATABASE_URL").expect(
			"Either set PGHOST + PGUSER + PGPASSWORD + PGDATABASE, \
			 or set DATABASE_URL",
		),
	}
}
