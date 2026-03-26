use sercli::Migrations;

#[test]
fn parses_petuh_relations() {
    let migrations = Migrations::get("../test_data/migrations").unwrap();

    let saved_response = migrations.entities.get("SavedResponse").unwrap();
    assert_eq!(saved_response.relations.len(), 2);
    assert!(
        saved_response
            .relations
            .iter()
            .any(|r| r.field == "user_id" && r.references == "User")
    );
    assert!(
        saved_response
            .relations
            .iter()
            .any(|r| r.field == "chat_id" && r.references == "Chat")
    );

    let user_stat = migrations.entities.get("UserStat").unwrap();
    assert_eq!(user_stat.relations.len(), 2);
    assert!(
        user_stat
            .relations
            .iter()
            .any(|r| r.field == "user_id" && r.references == "User")
    );
    assert!(
        user_stat
            .relations
            .iter()
            .any(|r| r.field == "chat_id" && r.references == "Chat")
    );

    let user = migrations.entities.get("User").unwrap();
    assert!(user.relations.is_empty());
}
