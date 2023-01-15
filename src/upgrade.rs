pub(crate) fn upgrade(bytes: Vec<u8>) -> Vec<u8> {
    upgrade(match bytes[0] {
        0 => upgrade_v1(bytes),
        1 => return bytes,
        v => panic!("invalid file version {v}")
    })
}

pub(crate) fn upgrade_v1(mut bytes: Vec<u8>) -> Vec<u8> {
    let _ = std::mem::replace(&mut bytes[0], 1);
    bytes.insert(2, 8);  // bps
    bytes.insert(3, 4);  // section_height
    bytes
}