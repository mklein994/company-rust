extern crate postgres;

use postgres::{Connection, TlsMode};

struct Employee {
    fname: String,
    //minit: String,
    //lname: String,
}

fn main() {
    let conn = Connection::connect("postgresql://u0_a88@localhost:5432/company", TlsMode::None).unwrap();

    for row in &conn.query("SELECT fname from employee;", &[]).unwrap() {
        let employee = Employee {
            fname: row.get(0),
            //minit: row.get(1),
            //lname: row.get(2),
        };

        println!("found Employee: {}", employee.fname);
    }

    println!("Hello, world!");
}
