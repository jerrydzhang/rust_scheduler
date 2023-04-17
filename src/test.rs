use tokio_rusqlite::Connection;
use crate::database::*;
use crate::events::{Event, UpdateEvent};




#[tokio::test]
async fn test_create_event() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = Event::new(1, 0, 0, "Test".to_string(), "Test".to_string()).await;

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,test_event).await.unwrap();

    let result = db_get_event(&conn, 1).await.unwrap();

    assert_eq!(result.id, 1);
    assert_eq!(result.start_time, 0);
    assert_eq!(result.end_time, 0);
    assert_eq!(result.title, "Test".to_string());
    assert_eq!(result.description, "Test".to_string());
}

#[tokio::test] 
async fn test_remove_event() {
    let conn = Connection::open_in_memory().await.unwrap();
    let test_event = Event::new(1, 0, 0, "Test".to_string(), "Test".to_string()).await;


    down(&conn).await.unwrap();
    up(&conn).await.unwrap();
    db_insert_event(&conn,test_event).await.unwrap();
    db_delete_event(&conn, 1).await.unwrap();
    let result = db_get_event(&conn, 1).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_event() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = Event::new(1, 0, 0, "Test".to_string(), "Test".to_string()).await;
    let updated_event = UpdateEvent::new(Some(1), Some(1), Some("Test2".to_string()), Some("Test2".to_string())).await;

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,test_event).await.unwrap();

    db_update_event(&conn, 1, updated_event).await.unwrap();

    let result = db_get_event(&conn, 1).await.unwrap();

    assert_eq!(result.id, 1);
    assert_eq!(result.start_time, 1);
    assert_eq!(result.end_time, 1);
    assert_eq!(result.title, "Test2".to_string());
    assert_eq!(result.description, "Test2".to_string());
}

#[tokio::test]
async fn test_list_events() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = Event::new(1, 0, 0, "Test".to_string(), "Test".to_string()).await;
    let test_event2 = Event::new(2, 0, 0, "Test2".to_string(), "Test2".to_string()).await;
    let test_event3 = Event::new(3, 3, 3, "number3".to_string(), "disc3".to_string()).await;

    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,test_event).await.unwrap();
    db_insert_event(&conn,test_event2).await.unwrap();
    db_insert_event(&conn,test_event3).await.unwrap();

    let result = db_list_events(&conn).await.unwrap();

    assert_eq!(result.len(), 3);

    assert_eq!(result[0].id, 1);
    assert_eq!(result[0].start_time, 0);
    assert_eq!(result[0].end_time, 0);
    assert_eq!(result[0].title, "Test".to_string());
    assert_eq!(result[0].description, "Test".to_string());

    assert_eq!(result[1].id, 2);
    assert_eq!(result[1].start_time, 0);
    assert_eq!(result[1].end_time, 0);
    assert_eq!(result[1].title, "Test2".to_string());
    assert_eq!(result[1].description, "Test2".to_string());

    assert_eq!(result[2].id, 3);
    assert_eq!(result[2].start_time, 3);
    assert_eq!(result[2].end_time, 3);
    assert_eq!(result[2].title, "number3".to_string());
    assert_eq!(result[2].description, "disc3".to_string());

}

#[tokio::test]
async fn combo_db_test() {
    let conn = Connection::open("test.db").await.unwrap();
    let test_event = Event::new(1, 0, 0, "Test".to_string(), "Test".to_string()).await;
    let updated_event = UpdateEvent::new(Some(1), Some(1), Some("Test2".to_string()), Some("Test2".to_string())).await;

    let test_event2 = Event::new(2, 0, 0, "Test".to_string(), "Test".to_string()).await;
    let test_event3 = Event::new(3, 3, 3, "number3".to_string(), "number3".to_string()).await;


    down(&conn).await.unwrap();
    up(&conn).await.unwrap();

    db_insert_event(&conn,test_event).await.unwrap();
    db_insert_event(&conn,test_event2).await.unwrap();
    db_insert_event(&conn,test_event3).await.unwrap();

    db_update_event(&conn, 1, updated_event).await.unwrap();

    let result = db_get_event(&conn, 1).await.unwrap();

    assert_eq!(result.id, 1);
    assert_eq!(result.start_time, 1);
    assert_eq!(result.end_time, 1);
    assert_eq!(result.title, "Test2".to_string());
    assert_eq!(result.description, "Test2".to_string());

    let result2 = db_get_event(&conn, 2).await.unwrap();

    assert_eq!(result2.id, 2);
    assert_eq!(result2.start_time, 0);
    assert_eq!(result2.end_time, 0);
    assert_eq!(result2.title, "Test".to_string());
    assert_eq!(result2.description, "Test".to_string());

    let result3 = db_get_event(&conn, 3).await.unwrap();

    assert_eq!(result3.id, 3);
    assert_eq!(result3.start_time, 3);
    assert_eq!(result3.end_time, 3);
    assert_eq!(result3.title, "number3".to_string());
    assert_eq!(result3.description, "number3".to_string());

    db_delete_event(&conn, 1).await.unwrap();
    let result = db_get_event(&conn, 1).await;

    assert!(result.is_err());
}
