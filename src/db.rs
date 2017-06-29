// infer_schema!("dotenv:DATABASE_URL");  // do NOT use this!
// 最好不要用 infer_schema，因为它是在编译时连接数据库，然后搞事情...
// 正式项目不应该这样做:
// 1. 考虑到多套环境 dev/prod
// 2. 考虑数据库变更的需求
//
// 这里显式声明 table 定义的方式更好一些


use diesel::Connection;
use diesel::mysql::MysqlConnection;
use constants::DATABASE_URL;

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(DATABASE_URL).unwrap()
}

// http://docs.diesel.rs/diesel/macro.table.html
table! {
    post {
        id      -> Integer,
        user_id -> Integer,
        url     -> VarChar,
        title   -> VarChar,
        content -> Text,
        created_at  -> Integer,
        updated_at  -> Integer,
        // is_archived -> Tiny,
        // is_archived tinyint(1) UNSIGNED NOT NULL DEFAULT 0,  -- 0: unread; 1: read
        // archived_at int(11) UNSIGNED NOT NULL,
    }
}
