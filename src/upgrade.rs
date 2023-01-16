pub(crate) fn upgrade(bytes: Vec<u8>) -> Vec<u8> {
    upgrade(match bytes[0] {
        0 => upgrade_v1(bytes),
        1 => upgrade_v2(bytes),
        2 => return bytes,
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
    let mut instrs: Vec<u8> = vec![];
    for _ in 0..6 { 
        // sine
        instrs.push(0); instrs.push(1);
        instrs.push(255);
        instrs.push(1);
    }
    for _ in 0..2 {
        // square
        instrs.push( 0); instrs.push(2);
        instrs.push(74);
        instrs.push(1);
        // saw
        instrs.push(0); instrs.push(3);
        instrs.push(74);
        instrs.push(1);
    }
    for _ in 0..4 {
        // triangle
        instrs.push(0); instrs.push(4);
        instrs.push(191);
        instrs.push(1);
    }
    let (pre, post) = bytes.split_at(4);
    let mut out = Vec::from(pre);
    out.append(&mut instrs);
    out.append(&mut Vec::from(post));
    out
}