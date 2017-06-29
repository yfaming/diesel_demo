use db::post;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Insertable)]
#[table_name="post"]
pub struct NewPost {
    pub user_id: i32,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: i32,
    pub updated_at: i32,
}
