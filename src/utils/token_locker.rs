pub fn serialize_data<T: BorshSerialize>(data: &T) -> Vec<u8> {
    borsh::to_vec(data).unwrap()
}

pub fn deserialize_data<T: BorshDeserialize>(data: &[u8]) -> T {
    borsh::from_slice(data).unwrap()
}
