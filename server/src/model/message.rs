// model/message.rs
use diesel::prelude::*;
use crate::schema::messages::{self};
use super::get_conn;

#[derive(Queryable, Insertable, AsChangeset, Debug, Clone)]
#[diesel(table_name = messages)]
pub struct Message {
    pub id: i32,
    pub sender: i32,
    pub receiver: i32,
    pub message_type: String,
    pub content: String,
    pub timestamp: chrono::NaiveDateTime,
    pub dead_time: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub sender: i32,
    pub receiver: i32,
    pub message_type: &'a str,
    pub content: &'a str,
    pub timestamp: chrono::NaiveDateTime,
    pub dead_time: chrono::NaiveDateTime,
}

impl Message {
    /// 获取所有消息
    pub fn get_all() -> Vec<Self> {
        use diesel::prelude::*;

        let mut conn = get_conn();
        messages::table.load::<Message>(&mut conn).unwrap()
    }

    /// 根据发送者和接收者获取消息
    pub fn get_by_sender_receiver(sender: i32, receiver: i32) -> Vec<Self> {
        use diesel::prelude::*;

        let mut conn = get_conn();
        messages::table
            .filter(
                (messages::sender.eq(sender)
                    .and(messages::receiver.eq(receiver)))
                    .or(messages::sender.eq(receiver)
                        .and(messages::receiver.eq(sender))),
            )
            .order(messages::timestamp.asc())
            .load::<Message>(&mut conn)
            .unwrap()
    }

    /// 创建新消息
    pub fn create(sender: i32, receiver: i32, message_type: &str, content: &str) -> Self {
        use diesel::prelude::*;
        use crate::util::APP_CONFIG;

        let mut conn = get_conn();
        // 获取当前时间
        let timestamp = chrono::Utc::now().naive_utc();
        let lifespan = APP_CONFIG.message.lifespan;
        let dead_time = timestamp + chrono::Duration::hours(lifespan);

        let new_message = NewMessage {
            sender,
            receiver,
            message_type,
            content,
            timestamp,
            dead_time,
        };

        diesel::insert_into(messages::table)
            .values(&new_message)
            .execute(&mut conn)
            .expect("Error creating message");

        messages::table
            .order(messages::id.desc())
            .first::<Message>(&mut conn)
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::messages;

    // 在测试前设置数据库连接并清理数据
    fn setup() {
        let mut conn = super::get_conn();
        diesel::delete(messages::table).execute(&mut conn).unwrap();
    }

    #[test]
    fn test_get_all_messages() {
        setup();
        let messages = Message::get_all();
        assert!(messages.is_empty());

        Message::create(1, 2, "text", "Hello, World!");
        let messages = Message::get_all();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello, World!");
    }

    #[test]
    fn test_get_messages_by_sender_receiver() {
        setup();
        Message::create(1, 2, "text", "Hello from 1 to 2");
        Message::create(2, 1, "text", "Hello from 2 to 1");

        let messages = Message::get_by_sender_receiver(1, 2);
        assert_eq!(messages.len(), 2);
        assert!(messages.iter().any(|m| m.content == "Hello from 1 to 2"));
        assert!(messages.iter().any(|m| m.content == "Hello from 2 to 1"));
    }

    #[test]
    fn test_create_message() {
        setup();
        let new_message = Message::create(1, 2, "text", "Hello, World!");
        assert_eq!(new_message.content, "Hello, World!");
        assert_eq!(new_message.sender, 1);
        assert_eq!(new_message.receiver, 2);
    }
}