extern crate diesel;
extern crate time;
use diesel::prelude::*;
use db::post;
use db::{establish_connection};
use dao::post::{Post, NewPost};


pub fn create_post(user_id: i32, url: String, title: String, content: String) {
    let now = time::now().to_timespec().sec as i32;
    let new_post = NewPost {
        user_id: user_id,
        url: url,
        title: title,
        content: content,
        created_at: now,
        updated_at: now,
    };

    // MySQL insert 时竟然拿不到 id。。。
    // 为啥 SQLAlchemy 就行呢？ `inserted_primary_key[0]` 这样
    let conn = establish_connection();
    let i = diesel::insert(&new_post).into(post::table)
        .execute(&conn)
        .expect("Error saving new post");
}

pub fn update_post(post_id: i32, title: String, content: String) {
    // 条件更新貌似比较麻烦，比如 title 和 content 为 Option 的情况
    let sql = diesel::update(post::table.filter(post::id.eq(post_id)))
        .set((post::title.eq(title), post::content.eq(content)));

    sql.execute(&establish_connection())
       .expect(&format!("Unable to find post {}", post_id));
}

pub fn delete_post(post_id: i32) {
    diesel::delete(post::table.filter(post::id.eq(post_id)))
        .execute(&establish_connection())
        .expect(&format!("Error when delete post {}", post_id));
}

pub fn get_unread_posts(user_id: i32, offset: i64, limit: i64) -> Vec<Post> {
    let conn = establish_connection();
    post::table.filter(post::user_id.eq(&user_id))
        .offset(offset)
        .limit(limit)
        .load::<Post>(&conn)
        .expect("error loading posts")
}
