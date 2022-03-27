#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use models::Saying;
use schema::says::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_random_saying() -> String {
    let connection = establish_connection();
    let total: i64 = says.count().first::<i64>(&connection).unwrap();
    let say: Saying = says
        .find(total as i32)
        .first::<Saying>(&connection)
        .unwrap();
    say.saying.to_owned()
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_get_random_saying() {
        assert_eq!("南无大方广佛华严经，华严海会佛菩萨", get_random_saying());
    }
}
