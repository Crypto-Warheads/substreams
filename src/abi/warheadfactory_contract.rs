    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct Claim {
            pub location: (substreams::scalar::BigInt, substreams::scalar::BigInt),
            pub warhead_id: substreams::scalar::BigInt,
        }
        impl Claim {
            const METHOD_ID: [u8; 4] = [253u8, 85u8, 82u8, 240u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Int(256usize),
                                    ethabi::ParamType::Int(256usize)
                                ],
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    location: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_signed_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_signed_bytes_be(&v)
                            },
                        )
                    },
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                { let non_full_signed_bytes = self.location.0
                                .to_signed_bytes_be(); let full_signed_bytes_init = if
                                non_full_signed_bytes[0] & 0x80 == 0x80 { 0xff } else { 0x00
                                }; let mut full_signed_bytes = [full_signed_bytes_init as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }, { let non_full_signed_bytes = self.location.1
                                .to_signed_bytes_be(); let full_signed_bytes_init = if
                                non_full_signed_bytes[0] & 0x80 == 0x80 { 0xff } else { 0x00
                                }; let mut full_signed_bytes = [full_signed_bytes_init as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }
                            ],
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.warhead_id.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for Claim {
            const NAME: &'static str = "claim";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateWarhead {
            pub target_receiver: Vec<u8>,
        }
        impl CreateWarhead {
            const METHOD_ID: [u8; 4] = [255u8, 8u8, 185u8, 250u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    target_receiver: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.target_receiver),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for CreateWarhead {
            const NAME: &'static str = "createWarhead";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct DropWarhead {
            pub warhead_id: substreams::scalar::BigInt,
            pub coord: (substreams::scalar::BigInt, substreams::scalar::BigInt),
            pub impact_time: substreams::scalar::BigInt,
        }
        impl DropWarhead {
            const METHOD_ID: [u8; 4] = [150u8, 94u8, 67u8, 80u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Int(256usize),
                                    ethabi::ParamType::Int(256usize)
                                ],
                            ),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    coord: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[0usize]
                                    .clone()
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_signed_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_int()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_signed_bytes_be(&v)
                            },
                        )
                    },
                    impact_time: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.warhead_id.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                        ethabi::Token::Tuple(
                            vec![
                                { let non_full_signed_bytes = self.coord.0
                                .to_signed_bytes_be(); let full_signed_bytes_init = if
                                non_full_signed_bytes[0] & 0x80 == 0x80 { 0xff } else { 0x00
                                }; let mut full_signed_bytes = [full_signed_bytes_init as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }, { let non_full_signed_bytes = self.coord.1
                                .to_signed_bytes_be(); let full_signed_bytes_init = if
                                non_full_signed_bytes[0] & 0x80 == 0x80 { 0xff } else { 0x00
                                }; let mut full_signed_bytes = [full_signed_bytes_init as
                                u8; 32]; non_full_signed_bytes.into_iter().rev().enumerate()
                                .for_each(| (i, byte) | full_signed_bytes[31 - i] = byte);
                                ethabi::Token::Int(ethabi::Int::from_big_endian(full_signed_bytes
                                .as_ref())) }
                            ],
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.impact_time.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
        }
        impl substreams_ethereum::Function for DropWarhead {
            const NAME: &'static str = "dropWarhead";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FetchWarheadInfo {
            pub warhead_id: substreams::scalar::BigInt,
        }
        impl FetchWarheadInfo {
            const METHOD_ID: [u8; 4] = [97u8, 156u8, 254u8, 112u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.warhead_id.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                (
                    Vec<u8>,
                    bool,
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    bool,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                ),
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<u8>,
                    bool,
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    bool,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                ),
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address, ethabi::ParamType::Bool,
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Int(256usize),
                                    ethabi::ParamType::Int(256usize)]),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address, ethabi::ParamType::Bool,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address
                                ],
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let tuple_elements = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        tuple_elements[1usize].clone().into_bool().expect(INTERNAL_ERR),
                        {
                            let tuple_elements = tuple_elements[2usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_int()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_int()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_signed_bytes_be(&v)
                                },
                            )
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[3usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        tuple_elements[4usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        tuple_elements[5usize].clone().into_bool().expect(INTERNAL_ERR),
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[6usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        tuple_elements[7usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                    )
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                (
                    Vec<u8>,
                    bool,
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    bool,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                ),
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FetchWarheadInfo {
            const NAME: &'static str = "fetchWarheadInfo";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            (
                Vec<u8>,
                bool,
                (substreams::scalar::BigInt, substreams::scalar::BigInt),
                substreams::scalar::BigInt,
                Vec<u8>,
                bool,
                substreams::scalar::BigInt,
                Vec<u8>,
            ),
        > for FetchWarheadInfo {
            fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<u8>,
                    bool,
                    (substreams::scalar::BigInt, substreams::scalar::BigInt),
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    bool,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                ),
                String,
            > {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IsTrustedForwarder {
            pub forwarder: Vec<u8>,
        }
        impl IsTrustedForwarder {
            const METHOD_ID: [u8; 4] = [87u8, 43u8, 108u8, 5u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    forwarder: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.forwarder),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IsTrustedForwarder {
            const NAME: &'static str = "isTrustedForwarder";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for IsTrustedForwarder {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OnErc721Received {
            pub param0: Vec<u8>,
            pub param1: Vec<u8>,
            pub param2: substreams::scalar::BigInt,
            pub param3: Vec<u8>,
        }
        impl OnErc721Received {
            const METHOD_ID: [u8; 4] = [21u8, 11u8, 122u8, 2u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Bytes,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    param2: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    param3: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bytes()
                        .expect(INTERNAL_ERR),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param0),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.param1),
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.param2.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                        ethabi::Token::Bytes(self.param3.clone()),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 4usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(4usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 4usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for OnErc721Received {
            const NAME: &'static str = "onERC721Received";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 4usize]> for OnErc721Received {
            fn output(data: &[u8]) -> Result<[u8; 4usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Owner {}
        impl Owner {
            const METHOD_ID: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Owner {
            const NAME: &'static str = "owner";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for Owner {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TrustedForwarder {}
        impl TrustedForwarder {
            const METHOD_ID: [u8; 4] = [125u8, 160u8, 168u8, 119u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for TrustedForwarder {
            const NAME: &'static str = "trustedForwarder";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for TrustedForwarder {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct VersionRecipient {}
        impl VersionRecipient {
            const METHOD_ID: [u8; 4] = [72u8, 111u8, 240u8, 205u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for VersionRecipient {
            const NAME: &'static str = "versionRecipient";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for VersionRecipient {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WarheadNft {}
        impl WarheadNft {
            const METHOD_ID: [u8; 4] = [192u8, 176u8, 189u8, 204u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Vec<u8>, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<Vec<u8>> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for WarheadNft {
            const NAME: &'static str = "warheadNft";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<Vec<u8>> for WarheadNft {
            fn output(data: &[u8]) -> Result<Vec<u8>, String> {
                Self::output(data)
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct WarheadClaimed {
            pub warhead_id: substreams::scalar::BigInt,
            pub claimer: Vec<u8>,
            pub claimed_at: substreams::scalar::BigInt,
        }
        impl WarheadClaimed {
            const TOPIC_ID: [u8; 32] = [
                111u8,
                178u8,
                27u8,
                9u8,
                14u8,
                245u8,
                168u8,
                1u8,
                120u8,
                217u8,
                47u8,
                174u8,
                38u8,
                46u8,
                26u8,
                94u8,
                97u8,
                141u8,
                154u8,
                227u8,
                94u8,
                19u8,
                253u8,
                160u8,
                239u8,
                215u8,
                103u8,
                122u8,
                76u8,
                128u8,
                63u8,
                183u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    claimer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    claimed_at: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for WarheadClaimed {
            const NAME: &'static str = "WarheadClaimed";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WarheadCreated {
            pub warhead_id: substreams::scalar::BigInt,
            pub dropper: Vec<u8>,
            pub warhead_address: Vec<u8>,
        }
        impl WarheadCreated {
            const TOPIC_ID: [u8; 32] = [
                238u8,
                132u8,
                78u8,
                169u8,
                37u8,
                83u8,
                137u8,
                37u8,
                44u8,
                57u8,
                223u8,
                111u8,
                155u8,
                214u8,
                55u8,
                171u8,
                65u8,
                176u8,
                59u8,
                225u8,
                226u8,
                93u8,
                114u8,
                77u8,
                219u8,
                194u8,
                17u8,
                10u8,
                197u8,
                140u8,
                25u8,
                23u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 96usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    dropper: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    warhead_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for WarheadCreated {
            const NAME: &'static str = "WarheadCreated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WarheadCreatedWithReceiver {
            pub warhead_id: substreams::scalar::BigInt,
            pub dropper: Vec<u8>,
            pub warhead_address: Vec<u8>,
            pub target_receiver: Vec<u8>,
        }
        impl WarheadCreatedWithReceiver {
            const TOPIC_ID: [u8; 32] = [
                177u8,
                23u8,
                88u8,
                139u8,
                109u8,
                181u8,
                155u8,
                203u8,
                249u8,
                236u8,
                130u8,
                186u8,
                95u8,
                50u8,
                178u8,
                111u8,
                212u8,
                103u8,
                120u8,
                135u8,
                82u8,
                122u8,
                15u8,
                47u8,
                12u8,
                86u8,
                142u8,
                170u8,
                92u8,
                181u8,
                251u8,
                242u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    dropper: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    warhead_address: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    target_receiver: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for WarheadCreatedWithReceiver {
            const NAME: &'static str = "WarheadCreatedWithReceiver";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct WarheadDropped {
            pub warhead_id: substreams::scalar::BigInt,
            pub target_lat: substreams::scalar::BigInt,
            pub target_long: substreams::scalar::BigInt,
            pub impact_time: substreams::scalar::BigInt,
        }
        impl WarheadDropped {
            const TOPIC_ID: [u8; 32] = [
                15u8,
                119u8,
                92u8,
                133u8,
                65u8,
                203u8,
                84u8,
                148u8,
                86u8,
                248u8,
                255u8,
                226u8,
                121u8,
                37u8,
                212u8,
                54u8,
                77u8,
                254u8,
                136u8,
                250u8,
                194u8,
                148u8,
                22u8,
                163u8,
                247u8,
                102u8,
                147u8,
                20u8,
                92u8,
                240u8,
                0u8,
                228u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Int(256usize),
                            ethabi::ParamType::Int(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    warhead_id: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    target_lat: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                    },
                    target_long: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_int()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_signed_bytes_be(&v)
                    },
                    impact_time: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for WarheadDropped {
            const NAME: &'static str = "WarheadDropped";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }