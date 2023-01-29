#![no_main]
use libfuzzer_sys::fuzz_target;

use bytes::BytesMut;
use mqtt_v5::{decoder, types::ProtocolVersion};

fuzz_target!(|data: &[u8]| {
    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(data);

    let _ = decoder::decode_mqtt(&mut bytes, ProtocolVersion::V500);
});
