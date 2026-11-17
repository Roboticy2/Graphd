use sqlite::*;

#[test]
fn test_in_memory_db() {
    //databases are a good candiate for my network datastructure
    // to make a relational db, simply open two tables and have them point to each other
    let conn = open(":memory:").unwrap();

    //my main question, is it possible to automate queries from struct shapes?
    let query = "
        create table users ('name' text, 'age' integer);
        insert into users values ('alice', 42);
        insert into users values ('bob', 69);
    ";

    assert!(match conn.execute(query) {
        Err(_) => false,
        _ => true,
    });

    // after some light googling, it looks like this library does't really do the whole "safety and automation" thing
    // rust is known for.
    // There's like 15 sql libraries. I tried one and it sent me on a wild goose chase to.. install sqlite.
    // I think I'm good here for now
    //
    // Overall, while I wanted to avoid rolling my own graph datastructutes, sql is a little heavy for the task.
}
