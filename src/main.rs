extern crate diesel_orm;

use diesel_orm::model::post::create_post;
use diesel_orm::model::post::delete_post;
use diesel_orm::model::post::get_unread_posts;
use diesel_orm::model::post::update_post;


fn main() {
    create_post(1,
                "https://en.wikipedia.org/wiki/Data_access_object".to_string(),
                "Data access object".to_string(),
                "In computer software, a data access object (DAO) is an object that...".to_string());

    update_post(45, "DAO".to_string(), "which I don't know".to_string());
    delete_post(45);

    for p in get_unread_posts(1, 0, 100) {
        println!("id={}, title={}", p.id, p.title);
    }
}
