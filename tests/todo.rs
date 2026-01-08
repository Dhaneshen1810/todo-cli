use todo::*;

#[test]
fn creates_new_todo() {
    let todo = Todo::new(1, "Learn Rust".to_string());

    assert_eq!(todo.id, "1");
    assert_eq!(todo.name, "Learn Rust");
}

#[test]
fn reset_ids_works() {
    let mut todos = vec![Todo::new(10, "A".into()), Todo::new(20, "B".into())];

    reset_todo_ids(&mut todos);

    assert_eq!(todos[0].id, "1");
    assert_eq!(todos[1].id, "2");
}
