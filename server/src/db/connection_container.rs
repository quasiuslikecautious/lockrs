// pub struct ConnectionContainer {
//     db_context: Arc<DbContext>,
//     pg: Option<AsyncPgConnection>,
//     redis: Option<AsyncRedisConnection>,
// }
//
// impl ConnectionContainer {
//     pub fn new(db_context: Arc<DbContext>) -> Self {
//         Self {
//             db_context: Arc::clone(db_context),
//             None,
//             None,
//         }
//     }
//
//     pub fn get_pg(&mut self) -> AsyncPgConnection {
//         if self.pg.is_none() {
//             self.pg = db_context.as_ref()
//                 .get_pg_connection()
//                 .await
//                 .map_err(|_| ConnectionContainerError::BadConnection);
//         }
//
//         self.pg.unwrap();
//     }
//
//     pub fn get_redis(&mut self) -> AsyncRedisConnection {
//
//     }
// }
//
// pub enum ConnectionContainerError {
//     BadConnection,
// }
