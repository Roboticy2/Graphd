use sqlx::SqliteConnection;

//A context for conversations and users
struct Context {
    conn: SqliteConnection, //Connection borrowed by children
    config: Config,         //Config
}

//Context settings
struct Config {}

//A user of conversations
struct User {}

//A relational db of hubs and their connections
struct Conversation {}

//A point in a conversation
struct Hub {}

//Tags relating to items with tags
pub trait TagDatabase {
    type It: Iterator;
    fn search(&self, tags: &Vec<String>) -> Self::It;
}
