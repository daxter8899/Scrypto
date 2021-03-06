pub mod simple;

pub fn get_simple_dataset(repeat: usize) -> simple::SimpleStruct {
    simple::SimpleStruct {
        number: 12345678901234567890,
        string: "dummy".repeat(repeat).to_owned(),
        vector1: vec![123u8; repeat],
        vector2: vec![12345u16; repeat],
        enumeration: simple::SimpleEnum::Named {
            x: 1234567890,
            y: 1234567890,
        },
    }
}
