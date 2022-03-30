#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use models::{BgImg, NewSaying, Saying};
use rand::Rng;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_random_saying() -> String {
    use schema::says::dsl::*;

    let mut rng = rand::thread_rng();
    let connection = establish_connection();
    let total: i64 = says.count().first::<i64>(&connection).unwrap();
    let randid: i32 = rng.gen_range(1..(total as i32 + 1));
    let say: Saying = says.find(randid).first::<Saying>(&connection).unwrap();
    say.saying.to_owned()
}

pub fn get_img_by_day(locate_day: u32, device_name: &str) -> String {
    use schema::bgimgs::dsl::*;
    let connection = establish_connection();
    bgimgs
        .filter(day.eq(locate_day as i32).and(device.eq(device_name)))
        .first::<BgImg>(&connection)
        .and_then(|bgimg_inst| Ok(bgimg_inst.url.to_owned()))
        .unwrap_or("".to_owned())
}

pub fn write_saying(conn: &SqliteConnection, say: String) {
    use schema::says::dsl::*;
    let new_saying = NewSaying {
        saying: say.as_str(),
    };
    diesel::insert_into(says)
        .values(&new_saying)
        .execute(conn)
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_get_random_saying() {
        write_saying(
            &establish_connection(),
            String::from("南无大方广佛华严经，华严海会佛菩萨"),
        );
        assert_eq!("南无大方广佛华严经，华严海会佛菩萨", get_random_saying());
    }

    #[test]
    fn test_get_img_by_day() {
        let url = get_img_by_day(30);
        assert_eq!(url, "test_url")
    }
}
