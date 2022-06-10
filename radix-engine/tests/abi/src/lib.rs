use sbor::describe::Fields;
use sbor::Type;
use scrypto::abi::{BlueprintAbi, Function};
use scrypto::prelude::*;

blueprint! {
    struct AbiComponent {}

    impl AbiComponent {
        pub fn create_component() -> ComponentAddress {
            Self {}.instantiate().globalize()
        }

        pub fn create_invalid_abi_component() -> ComponentAddress {
            Self {}
                .instantiate()
                .add_access_check(
                    AccessRules::new()
                        .method("no_method", rule!(require("something")))
                        .default(rule!(allow_all)),
                )
                .globalize()
        }
    }
}

#[no_mangle]
pub extern "C" fn AbiComponent2_main(_input: *mut u8, _input2: *mut u8) -> *mut u8 {
    ::scrypto::buffer::scrypto_encode_to_buffer(&())
}

#[no_mangle]
pub extern "C" fn AbiComponent2_abi(_input: *mut u8, _input2: *mut u8) -> *mut u8 {
    let value = Type::Struct {
        name: "AbiComponent2".to_string(),
        fields: Fields::Unit,
    };
    let abi = BlueprintAbi {
        value,
        functions: vec![
            Function {
                name: "unit".to_string(),
                mutability: Option::None,
                input: Type::Unit,
                output: Type::Unit,
            },
            Function {
                name: "bool".to_string(),
                mutability: Option::None,
                input: Type::Bool,
                output: Type::Unit,
            },
            Function {
                name: "i8".to_string(),
                mutability: Option::None,
                input: Type::I8,
                output: Type::Unit,
            },
            Function {
                name: "i16".to_string(),
                mutability: Option::None,
                input: Type::I16,
                output: Type::Unit,
            },
            Function {
                name: "i32".to_string(),
                mutability: Option::None,
                input: Type::I32,
                output: Type::Unit,
            },
            Function {
                name: "i64".to_string(),
                mutability: Option::None,
                input: Type::I64,
                output: Type::Unit,
            },
            Function {
                name: "i128".to_string(),
                mutability: Option::None,
                input: Type::I128,
                output: Type::Unit,
            },
            Function {
                name: "u8".to_string(),
                mutability: Option::None,
                input: Type::U8,
                output: Type::Unit,
            },
            Function {
                name: "u16".to_string(),
                mutability: Option::None,
                input: Type::U16,
                output: Type::Unit,
            },
            Function {
                name: "u32".to_string(),
                mutability: Option::None,
                input: Type::U32,
                output: Type::Unit,
            },
            Function {
                name: "u64".to_string(),
                mutability: Option::None,
                input: Type::U64,
                output: Type::Unit,
            },
            Function {
                name: "u128".to_string(),
                mutability: Option::None,
                input: Type::U128,
                output: Type::Unit,
            },
            Function {
                name: "result".to_string(),
                mutability: Option::None,
                input: Type::Result {
                    okay: Box::new(Type::Unit),
                    error: Box::new(Type::Unit),
                },
                output: Type::Unit,
            },
            Function {
                name: "tree_map".to_string(),
                mutability: Option::None,
                input: Type::TreeMap {
                    key: Box::new(Type::Unit),
                    value: Box::new(Type::Unit),
                },
                output: Type::Unit,
            },
            Function {
                name: "hash_set".to_string(),
                mutability: Option::None,
                input: Type::HashSet {
                    element: Box::new(Type::Unit),
                },
                output: Type::Unit,
            },
        ],
    };

    ::scrypto::buffer::scrypto_encode_to_buffer(&abi)
}