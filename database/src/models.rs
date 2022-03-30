use super::schema::says;
#[derive(Queryable)]
pub struct Saying {
    pub id: i32,
    pub saying: String,
}

#[derive(Insertable)]
#[table_name = "says"]
pub struct NewSaying<'a> {
    pub saying: &'a str,
}

#[derive(Queryable)]
pub struct BgImg {
    pub id: i32,
    pub day: i32,
    pub device: String,
    pub url: String
}
