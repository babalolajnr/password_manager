use database::init::init;
use futures::executor::block_on;

mod database;

#[actix_web::main]
async fn main() {
    // Connect to database
    if let Err(err) = block_on(init()) {
        panic!("{}", err);
    }
}
