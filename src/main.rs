
extern crate spider;
extern crate rusqlite;

use rusqlite::Connection;
use spider::website::Website;



fn main() {
    let conn = Connection::open("spider.sqlite").unwrap();

    conn.execute(
        "CREATE TABLE pages (id INTEGER PRIMARY KEY, url TEXT NOT NULL, html TEXT NOT NULL)",
        &[],
    ).unwrap();

    let mut localhost = Website::new("http://rousseau-alexandre.fr");
    localhost.crawl();

    for page in localhost.get_pages() {
        conn.execute(
            "INSERT INTO pages (url, html) VALUES (?1, ?2)",
            &[&page.get_url(), &page.get_plain_html()],
        ).unwrap();
    }

}
