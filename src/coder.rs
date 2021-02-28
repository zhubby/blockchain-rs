use bincode;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub fn serialize<T: ?Sized>(value: &T) -> Vec<u8>
    where T: Serialize,
{
    let seialized = bincode::serialize(value).unwrap();
    seialized
}

pub fn deserialize<'a, T>(bytes: &'a[u8]) -> T
    where T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8], mut out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result(&mut out);
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coder_works() {
        let point = Point{ x: 1, y: 1 };
        let se = serialize(&point);
        let de: Point = deserialize(&se);

        assert_eq!(de, point);
    }
}