//use log::{debug, error, info, trace, warn};
//use rusqlite::types::ToSql;

mod database;

fn main() {
    let conn = match database::SQLiteConnection::new()
        .with_path("db")
        .connect()
    {
        Ok(conn) => conn,
        Err(e) => {
            panic!("No database connection available: {}", e);
        }
    };
    let mut db = database::Manager::new(&conn);
    db.create_tables();
    // Test inserting some fake card data
    db.insert_card(
        &[
        &rusqlite::types::Null,
        &"Ability",
        &"Artist",
        &0,
        &1.0,
        &1,
        &"Color",
        &"Flavour",
        &vec![0u8],
        &1,
        &"Language",
        &1,
        &"Loyalty",
        &"Mana Cost",
        &0,
        &"Name",
        &"Number",
        &"Power",
        &2.0,
        &0,
        &"Rarity",
        &"Scryfall_ID",
        &"Set_Name",
        &"Set_Code",
        &"Toughness",
        &"Type Line",
        &"Watermark",
        ]
    ).expect("Failed to insert card.");
}
