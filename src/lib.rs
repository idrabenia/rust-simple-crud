pub mod schema;
pub mod models;
pub mod connection;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_export]
macro_rules! res {
    ( $type:ty ) => ( Result<Json<$type>, Status> );
}

#[macro_export]
macro_rules! res_vec {
    ( $type:ty ) => ( Result<Json<Vec<$type>>, Status> );
}
