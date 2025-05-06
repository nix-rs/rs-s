use rusqlite::{Connection, Result};
use std::error::Error;
use uuid::Uuid;


// Database
// -- if there is no db create the db
//          -- sites and app state db
//          -- articles db
// 
// -- edit in the main db
// -- edit in the articles db
//
// -- Add to main db -- check for 200OK before added to db
// -- Add to the article db
//

#[derive(Debug)]
struct Rss {
    id: i32,
    link: String,
}


pub fn add_links(l: &Vec<String>) -> Result<()> {
    //let id = Uuid::new_v4();


    let path = "./check-db.db3";
    let conn = Connection::open(path).expect("Connection Refuesd.");

    //&l.iter().for_each(|x| println!("{x}"));

    conn.execute(
        "CREATE TABLE IF NOT EXISTS rss (
            id  INTEGER PRIMARY KEY,
            link TEXT NOT NULL
        )",
        (),
    ).expect("Execution 'CREATE' is failed.");

    for links in l {
        conn.execute(
            "
                INSERT INTO rss (link) VALUES (?1)",
                (&links,)
                ).expect("Execution 'INSERT' is failed.");
        }

    Ok(())
}

pub fn update_links(id: &Vec<String>) -> Result<()> {
    let path = "./check-db.db3";
    let conn = Connection::open(path).expect("Connection Refuesd.");
        
    conn.execute(
            "
                UPDATE rss SET link = (?2) WHERE id = (?1)",
                (&id[0], &id[1])
                ).expect("Execution 'INSERT' is failed.");

    Ok(())
}

pub fn delete_links(id: &i32) -> Result<()> {
    let path = "./check-db.db3";
    let conn = Connection::open(path).expect("Connection Refuesd.");

    conn.execute(
            "
                DELETE FROM rsS WHERE id = (?1)",
                (&id,)
                ).expect("Execution 'INSERT' is failed.");

    Ok(())

}

// ERROR ERROR THIS IS NOT WORKING
pub fn view_links() {
    let path = "./check-db.db3";
    let conn = Connection::open(path).expect("Connection Refuesd.");

    let mut view = conn.prepare("SELECT id, link FROM rss").unwrap();
    let iters = view.query_map([], |row| {
        Ok(Rss {
            id: row.get(0).unwrap(),
            link: row.get(1).unwrap(),
        })
    }).unwrap();

    for v in iters {
        println!("views: {:?}",v.unwrap())
    }

}
