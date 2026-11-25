use actix_web::{
    error::{ErrorInternalServerError, ErrorServiceUnavailable},
    web,
};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use log::error;

pub struct ServerState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub secret: String,
}

impl ServerState {
    pub async fn conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
        let pool = self.pool.clone();

        web::block(move || pool.get())
            .await
            .map_err(ErrorInternalServerError)?
            .map_err(|err| {
                error!("No database connection available: {err}");
                ErrorServiceUnavailable(err)
            })
    }
}
