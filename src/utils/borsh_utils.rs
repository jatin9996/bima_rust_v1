pub fn serialize<T: BorshSerialize>(item: &T) -> Vec<u8> {
    borsh::serialize(item).unwrap()
}

pub fn deserialize<T: BorshDeserialize>(data: &[u8]) -> T {
    borsh::deserialize(data).unwrap()
}
