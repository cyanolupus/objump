use std::{
    borrow::BorrowMut, collections::HashMap, io::{self, Write}
};
use rusqlite::{self, Connection, Params, Statement};

#[derive(Debug)]
struct Person {
    id: i64,
    name: String,
    data: Option<Vec<u8>>,
}

impl TryFrom<&rusqlite::Row<'_>> for Person {
    type Error = rusqlite::Error;

    fn try_from(row: &rusqlite::Row) -> rusqlite::Result<Person> {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    }
}

fn main() -> Result<(), rusqlite::Error> {
    let conn = rusqlite::Connection::open("test.db")?;

    let mut stmts = (0..10).map(|i| {
        conn.prepare("select id, name, data from person where id < ?1 and id >= ?2").unwrap()
    }).collect::<Vec<Statement>>();

    let mut person_iter_map = HashMap::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        let person_iter = PersonIterator::new(stmt, ((i+1)*10, i*10));
        person_iter_map.insert(i, person_iter);
    }

    for (i, person_iter) in person_iter_map {
        println!("Found person {:?}", i);
        for person in person_iter {
            println!("{:?}", person);
        }
    }

    Ok(())
}

struct PersonIterator<'conn> {
    rows: rusqlite::Rows<'conn>,
}

impl<'conn> Iterator for PersonIterator<'conn> {
    type Item = Person;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rows.next().ok()??.try_into().ok()?)
    }
}

impl<'conn> PersonIterator<'conn> {
    fn new<P: Params>(stmt: &'conn mut rusqlite::Statement, params: P) -> PersonIterator<'conn> {
        PersonIterator{
            rows: stmt.query(params).unwrap()
        }
    }
}
