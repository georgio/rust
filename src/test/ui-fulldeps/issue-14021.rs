// run-pass

#![allow(unused_mut)]
#![allow(unused_imports)]
#![feature(rustc_private)]

extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct UnitLikeStruct;

pub fn main() {
    let obj = UnitLikeStruct;
    let json_str: String = json::encode(&obj).unwrap();

    let json_object = json::from_str(&json_str);
    let mut decoder = json::Decoder::new(json_object.unwrap());
    let mut decoded_obj: UnitLikeStruct = Decodable::decode(&mut decoder).unwrap();

    assert_eq!(obj, decoded_obj);
}
