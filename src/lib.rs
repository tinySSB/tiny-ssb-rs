pub mod feed_id;
pub mod go_set;

// use go_set::GOSetClaim;

#[derive(Debug)]
pub enum Error {}

// enum WirePacket {
//     // Dunno about this. Might be bytes not sure
//     Replication(GOSetClaim),
//     Log(),
// }

#[cfg(test)]
mod tests {
    // use super::*;
    use bipf_rs::bipf::{Bipf, decode};

    #[test]
    fn round_trip() {
        let value = serde_json::Value::String("Mix is cool".to_string());

        let value_bytes = value.to_bipf();
        println!("{:?}", value_bytes);

        let decoded = decode(&value_bytes).unwrap();

        assert_eq!(value, decoded);
    }
}
