use crate::store::LoaDb;
pub mod store;

fn main() {
    let mut db = LoaDb::new();

    db.put("name".into(), "Alexandria".into());
    println!("{:?}", db.get("name")); // Some("Alexandria")

    db.delete("name");
    println!("{:?}", db.get("name")); // None
}
