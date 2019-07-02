extern crate prrr_gql;
extern crate diesel;

use self::prrr_gql::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use prrr_gql::schema::cats::dsl::*;
    use prrr_gql::schema::birds::dsl::*;

    let connection = establish_connection();
    let results = cats.load::<Cat>(&connection)
        .expect("Error hearding cats");

    for cat in results {
        println!("{}", cat.name);

    }
}
