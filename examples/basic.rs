use rdom::{Dom, Selector};

fn main() {
    println!("🗓️  Parsing html...");
    let dom = Dom::from_string(include_str!("../test.html").to_string());

    println!("🔎 Searching for list links...");
    let links = dom.query(&[
        Selector::Tag("body"),
        Selector::AnyChild,
        Selector::Tag("nav"),
        Selector::AnyChild,
        Selector::Tag("a"),
    ]); // );

    println!("✅ Done\n");

    for link in links {
        println!("  {}", link.get_text().trim());
    }
}
