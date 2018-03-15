
extern crate spider;
extern crate rusqlite;

use rusqlite::Connection;
use spider::website::Website;



fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE pages (id INTEGER PRIMARY KEY, url TEXT NOT NULL)",
        &[],
    ).unwrap();

    let mut localhost = Website::new("http://rousseau-alexandre.fr");
    localhost.crawl();

    for page in localhost.get_pages() {
        conn.execute("INSERT INTO pages (url) VALUES (?1)", &[&page.get_url()])
            .unwrap();
    }

}
