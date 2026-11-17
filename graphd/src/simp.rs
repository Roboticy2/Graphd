use sqlite::{self, Error};

//implement basic sql queries for Self
pub trait SimpQuery: Clone {
    type It: Iterator;

    // (safely) register a table of this type to the connection
    fn create(c: &sqlite::Connection) -> Result<(), Error>;

    // opposite of create
    fn drop(c: &sqlite::Connection) -> Result<(), Error>;

    // insert a clone of self into the table if created
    fn insert(&self, c: &sqlite::Connection) -> Result<(), Error>;

    // opposite of insert
    fn delete(&self, c: &sqlite::Connection) -> Result<(), Error>;

    // Basically the example from the homepage of the docs https://docs.rs/sqlite/latest/sqlite/
    // but instead of looping through the cursor immediately, use map to ret a user defined iterator.
    fn map(c: sqlite::Connection, q: &str) -> Result<Self::It, Error>;
}
