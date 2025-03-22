// model/user.rs

use diesel::prelude::*;
use crate::schema::users::{self};
use crate::model::get_conn;

// 定义 User 模型
#[derive(Queryable, Insertable, AsChangeset, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub status: bool,
}

// 新增用户
#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub status: bool,
}

// 更新用户状态
#[derive(AsChangeset, Debug, Clone)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub status: bool,
}

impl User {
    /// 获取所有用户
    pub fn get_all() -> Vec<Self> {
        let mut conn = get_conn();
        users::table.load::<User>(&mut conn).unwrap()
    }

    /// 根据用户 ID 获取用户
    pub fn get_by_id(user_id: i32) -> Option<Self> {
        let mut conn = get_conn();
        users::table.find(user_id).first::<User>(&mut conn).ok()
    }

    /// 创建新用户
    pub fn create(username: &str, password: &str) -> Self {
        use diesel::prelude::*;

        let mut conn = get_conn();
        let new_user = NewUser {
            username,
            password,
            status: true,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)
            .expect("Error creating user");

        users::table.order(users::user_id.desc()).first::<User>(&mut conn).unwrap()
    }

    /// 更新用户状态
    pub fn update_status(user_id: i32, _status: bool) -> bool {
        use diesel::prelude::*;

        let mut conn = get_conn();
        let update_user = UpdateUser { status: _status };

        diesel::update(users::table.find(user_id))
            .set(&update_user)
            .execute(&mut conn)
            .unwrap() > 0
    }

    /// 删除用户
    pub fn delete(user_id: i32) -> bool {
        use diesel::prelude::*;

        let mut conn = get_conn();
        diesel::delete(users::table.find(user_id))
            .execute(&mut conn)
            .unwrap() > 0
    }
}


// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::get_conn;

    // 在测试前设置数据库连接并清理数据
    fn setup() {
        let mut conn = get_conn();
        diesel::delete(users::table).execute(&mut conn).unwrap();
    }

    #[test]
    fn test_get_all_users() {
        setup();
        let users = User::get_all();
        assert!(users.is_empty());

        User::create("test_user", "password");
        let users = User::get_all();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "test_user");
    }

    #[test]
    fn test_get_user_by_id() {
        setup();
        let new_user = User::create("test_user", "password");
        let user = User::get_by_id(new_user.user_id);
        assert!(user.is_some());
        assert_eq!(user.unwrap().username, "test_user");
    }

    #[test]
    fn test_create_user() {
        setup();
        let new_user = User::create("test_user", "password");
        assert_eq!(new_user.username, "test_user");
        assert_eq!(new_user.status, true);
    }

    #[test]
    fn test_update_user_status() {
        setup();
        let new_user = User::create("test_user", "password");
        let updated = User::update_status(new_user.user_id, false);
        assert!(updated);

        let user = User::get_by_id(new_user.user_id).unwrap();
        assert_eq!(user.status, false);
    }

    #[test]
    fn test_delete_user() {
        setup();
        let new_user = User::create("test_user", "password");
        let deleted = User::delete(new_user.user_id);
        assert!(deleted);

        let user = User::get_by_id(new_user.user_id);
        assert!(user.is_none());
    }
}