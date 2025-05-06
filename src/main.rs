use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let path = "./check-db.db3";
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE person (
            id  INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )?;

    let me = Person {
        id: 0,
        name: "Stevme".to_string(),
        data: None,
    };

    conn.execute(
        "
            INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare(
        "
        SELECT id, name, data FROM person",
    )?;

    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("FOund person  {:?}", person.unwrap());
    }

    Ok(())
}

//use rss::Channel;
//use std::error::Error;

// TODO
// - Add rss sites ( also check that link is valid before adding to database )
// - View all the rss sites
// - Remove rss sites
//
// - Fetching

/*
async fn exam() -> Result<Channel, Box<dyn Error>> {
    let url = "https://techcrunch.com/feed";
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}


#[tokio::main]
async fn main() {
    let ch = exam().await.unwrap();
    ch.items
        .iter()
        .for_each(|x| println!("{:#?}", x.title.as_ref().unwrap()));
}


use owo_colors::OwoColorize;

fn main() {
    println!("My number is {:#x}!", 10.green());
}

*/
