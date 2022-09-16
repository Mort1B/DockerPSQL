use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://admin:password@localhost:5432/postgres", NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    )?;

    client.execute(
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
        &[&"username", &"pw", &"test@gmail.com"],
    )?;

    // client.execute(
    //     "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
    //     &[&"username2", &"pass2", &"myemail@gmail.com"],
    // )?;

    // client.execute(
    //     "INSERT INTO users (username, password, email) VALUES ($1, $2, $3)",
    //     &[&"username3", &"pword3", &"mysecondemail@gmail.com"],
    // )?;

    for row in client.query("SELECT id, username, password, email FROM users", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);

        println!(
            "Id: {},\n username: {},\n password{},\n email: {},\n",
            id, username, password, email
        );
    }

    client.execute(
        "UPDATE users SET username=$1 WHERE id=$2",
        &[&"paulwall", &"3".parse::<i32>().unwrap()],
    )?;

    client.execute(
        "DELETE FROM users WHERE id=$1",
        &[&"1".parse::<i32>().unwrap()],
    )?;

    Ok(())
}
