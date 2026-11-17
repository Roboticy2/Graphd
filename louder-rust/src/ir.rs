use sqlite::{Connection, State};
use graphd::simp::*;

//A context for conversations and users
struct Context {
    conn: Connection, //Connection borrowed by children
    config: Config,   //Config
}

//Context settings
struct Config {}

//A user of conversations
struct User {}

//A relational db of hubs and their connections
struct Conversation {}

//A point in a conversation
#[derive(Clone)]
struct Hub {
    username: String,
    text: String,
    priority: int,
}

impl SimpQuery for Hub {
    fn create(c: &sqlite::Connection) -> Result<(), sqlite::Error> {
        let query = "
            create table if not exists hub (
                username text,
                _text text,
            );
                
        ";
        c.execute(query)
    }

    fn drop(c: &sqlite::Connection) -> Result<(), sqlite::Error> {
        let query = "
            drop table if exists hub;
        ";
        c.execute(query)
    }

    fn insert(&self, c: &sqlite::Connection) -> Result<(), sqlite::Error> {
        let query = "
            insert into hub (
                username, _text
            ) values (
                ?, ?
            );
        ";

        let mut stmt = c.prepare(query)?;
        stmt.bind((1, self.username.as_str()))?;
        stmt.bind((2, self.text.as_str())?;
        while let Ok(State::Row) = stmt.next() {
            
        }
        Ok(())
    }
}
