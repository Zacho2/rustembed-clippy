use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "testdata/"]
struct TestData;

fn main() {
    println!("Hello, world!");
    let test = TestData::get("test.json").unwrap();
    println!("{:?}", std::str::from_utf8(test.data.as_ref()));
}
