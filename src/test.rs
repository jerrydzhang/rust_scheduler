use tokio_rusqlite::Connection;
use crate::database::database_functions::{up, down};
use crate::database::event::event_functions::*;
use crate::database::event::event_structs::{UpdateEvent, CreateEvent};
use crate::database::user::user_structs::User;


#[tokio::test]
async fn test_create_event() {
    let conn = Connection::open("test.db").await.unwrap();

    let user = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 1,
        token: "testtoken".to_string(),
    };

    let test_event = CreateEvent::new("2021-10-13 00:00:00".to_string(), "2021-10-13 10:00:00".to_string(), "Test".to_string(), "Test".to_string()).await;

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,user.id ,test_event).await.unwrap();

    let result = db_get_event(&conn, user.id, 0).await.unwrap();

    assert_eq!(result.id, 0);
    assert_eq!(result.start_time, "2021-10-13 00:00:00".to_string());
    assert_eq!(result.end_time, "2021-10-13 10:00:00".to_string());
    assert_eq!(result.title, "Test".to_string());
    assert_eq!(result.description, "Test".to_string());
}

#[tokio::test] 
async fn test_remove_event() {
    let conn = Connection::open_in_memory().await.unwrap();
    let test_event = CreateEvent::new("2021-10-13 00:00:00".to_string(), "2021-10-13 10:00:00".to_string(), "Test".to_string(), "Test".to_string()).await;

    let user = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 1,
        token: "testtoken".to_string(),
    };

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();
    db_insert_event(&conn,user.id,test_event).await.unwrap();
    db_delete_event(&conn,user.id, 0).await.unwrap();
    let result = db_get_event(&conn,user.id, 0).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_event() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = CreateEvent::new("2021-10-13 00:00:00".to_string(), "2021-10-13 10:00:00".to_string(), "Test".to_string(), "Test".to_string()).await;
    let updated_event = UpdateEvent::new(None, Some("2021-10-13 23:00:00".to_string()), None, Some("Test2".to_string())).await;

    let user = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 1,
        token: "testtoken".to_string(),
    };

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,user.id,test_event).await.unwrap();

    db_update_event(&conn,user.id, 0, updated_event).await.unwrap();

    let result = db_get_event(&conn,user.id, 0).await.unwrap();

    assert_eq!(result.id, 0);
    assert_eq!(result.start_time, "2021-10-13 00:00:00".to_string());
    assert_eq!(result.end_time, "2021-10-13 23:00:00".to_string());
    assert_eq!(result.title, "Test".to_string());
    assert_eq!(result.description, "Test2".to_string());
}

#[tokio::test]
async fn test_list_events() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = CreateEvent::new("2021-10-13 00:00:00".to_string(), "2021-10-13 10:00:00".to_string(), "Test".to_string(), "Test".to_string()).await;
    let test_event2 = CreateEvent::new("2021-10-13 01:00:00".to_string(), "2021-10-13 10:02:00".to_string(), "Test2".to_string(), "Test2".to_string()).await;
    let test_event3 = CreateEvent::new("2021-10-13 03:33:33".to_string(), "2021-10-13 10:00:03".to_string(), "number3".to_string(), "disc3".to_string()).await;


    let user1 = User{
        username: "testuser1".to_string(),
        password: "testpass1".to_string(),
        id: 1,
        token: "testtoken1".to_string(),
    };

    let user2 = User{
        username: "testuser2".to_string(),
        password: "testpass2".to_string(),
        id: 2,
        token: "testtoken2".to_string(),
    };

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,user1.id,test_event).await.unwrap();
    db_insert_event(&conn,user1.id,test_event2).await.unwrap();
    db_insert_event(&conn,user2.id,test_event3).await.unwrap();

    let result1 = db_list_events(&conn,user1.id).await.unwrap();

    let result2 = db_list_events(&conn,user2.id).await.unwrap();

    assert_eq!(result1.len(), 2);

    assert_eq!(result1[0].id, 0);
    assert_eq!(result1[0].start_time, "2021-10-13 00:00:00".to_string());
    assert_eq!(result1[0].end_time, "2021-10-13 10:00:00".to_string());
    assert_eq!(result1[0].title, "Test".to_string());
    assert_eq!(result1[0].description, "Test".to_string());

    assert_eq!(result1[1].id, 1);
    assert_eq!(result1[1].start_time, "2021-10-13 01:00:00".to_string());
    assert_eq!(result1[1].end_time, "2021-10-13 10:02:00".to_string());
    assert_eq!(result1[1].title, "Test2".to_string());
    assert_eq!(result1[1].description, "Test2".to_string());

    assert_eq!(result2[0].id, 2);
    assert_eq!(result2[0].start_time, "2021-10-13 03:33:33".to_string());
    assert_eq!(result2[0].end_time, "2021-10-13 10:00:03".to_string());
    assert_eq!(result2[0].title, "number3".to_string());
    assert_eq!(result2[0].description, "disc3".to_string());

}

#[tokio::test]
async fn combo_db_test() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = CreateEvent::new("2021-10-13 00:00:00".to_string(), "2021-10-13 10:00:00".to_string(), "Test".to_string(), "Test".to_string()).await;
    let updated_event = UpdateEvent::new(None, Some("2021-10-13 23:00:00".to_string()), None, Some("Test2".to_string())).await;

    let test_event2 = CreateEvent::new("2021-10-13 01:00:00".to_string(), "2021-10-13 10:02:00".to_string(), "Test2".to_string(), "Test2".to_string()).await;
    let test_event3 = CreateEvent::new("2021-10-13 03:33:33".to_string(), "2021-10-13 10:00:03".to_string(), "number3".to_string(), "disc3".to_string()).await;

    let user1 = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 1,
        token: "testtoken".to_string(),
    };

    let user2 = User{
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        id: 1,
        token: "testtoken".to_string(),
    };

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,user1.id,test_event).await.unwrap();
    db_insert_event(&conn,user1.id,test_event2).await.unwrap();
    db_insert_event(&conn,user2.id,test_event3).await.unwrap();

    db_update_event(&conn,user1.id, 0, updated_event).await.unwrap();

    let result = db_get_event(&conn,user1.id, 0).await.unwrap();

    assert_eq!(result.id, 0);
    assert_eq!(result.start_time, "2021-10-13 00:00:00".to_string());
    assert_eq!(result.end_time, "2021-10-13 23:00:00".to_string());
    assert_eq!(result.title, "Test".to_string());
    assert_eq!(result.description, "Test2".to_string());

    let result2 = db_get_event(&conn,user1.id, 1).await.unwrap();

    assert_eq!(result2.id, 1);
    assert_eq!(result2.start_time, "2021-10-13 01:00:00".to_string());
    assert_eq!(result2.end_time, "2021-10-13 10:02:00".to_string());
    assert_eq!(result2.title, "Test2".to_string());
    assert_eq!(result2.description, "Test2".to_string());

    let result3 = db_get_event(&conn,user2.id, 2).await.unwrap();

    assert_eq!(result3.id, 2);
    assert_eq!(result3.start_time, "2021-10-13 03:33:33".to_string());
    assert_eq!(result3.end_time, "2021-10-13 10:00:03".to_string());
    assert_eq!(result3.title, "number3".to_string());
    assert_eq!(result3.description, "disc3".to_string());

    db_delete_event(&conn,user1.id, 0).await.unwrap();
    let result = db_get_event(&conn,user1.id, 0).await;

    assert!(result.is_err());
}
