pub(crate) fn upgrade(bytes: Vec<u8>) -> Vec<u8> {
    upgrade(match bytes[0] {
        0 => upgrade_v1(bytes),
        99 => upgrade_v2(bytes),
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

pub(crate) fn upgrade_v2(mut bytes: Vec<u8>) -> Vec<u8> {
    let _ = std::mem::replace(&mut bytes[0], 2);
    assert_eq!(bytes[1], 14, "invalid number of instruments while upgrading to file format 2: {}", bytes[1]);
    for _ in 0..6 {
        bytes.insert(4, 0);
        bytes.insert(4, 255);
    }
    bytes
}