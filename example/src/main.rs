use sqlite::State;

fn main() {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('루나', 3);
        INSERT INTO users VALUES ('러스트', 13);
    ";

    connection.execute(query).unwrap();

    let query = "SELECT * FROM users where age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 5)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}
