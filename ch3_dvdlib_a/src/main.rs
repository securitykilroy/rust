extern crate rustc_serialize;
use rustc_serialize::json::{self,ToJson, Json};

#[derive(RustcEncodable)]
struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16
}

impl ToJson for Dvd {
    fn to_json(&self) -> Json {
        Json::String(format!("{}+{}+{}+{}i", self.name, self.year, self.cast, self.length))
    }
}

fn converttojson (advd: &Dvd) -> String {

    json::encode(advd).unwrap()

}

fn main() {

    let a = Dvd {
        name: String::from("Four Weddings and a Funeral"),
        year: 1994,
        cast: String::from("Hugh Grant"),
        length: 117,
    };

    let encoded = converttojson(&a);
    println!("{}", encoded);

}
