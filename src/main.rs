#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    let coll = client.db("media").collection("movies");
    coll.insert_one(doc!{ "title": "Back to the Future" }, None).unwrap();
    coll.update_one(doc!{}, doc!{ "director": "Robert Zemeckis" }, None).unwrap();
    coll.delete_many(doc!{}, None).unwrap();

    let mut cursor = coll.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref title)) = item.get("title") {
                println!("title: {}", title);
            }
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}