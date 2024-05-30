use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint256;

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct HashList {
    pub hashes: Vec<Uint256>,
}

impl HashList {
    pub fn clear(&mut self) {
        self.hashes.clear();
    }
}

pub const HASH_LIST: Item<HashList> = Item::new("HashList");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_list() {
        let mut state = HashList {
            hashes: vec![
                Uint256::from_u128(1_u128), Uint256::from_u128(2_u128),
                Uint256::from_u128(3_u128), Uint256::from_u128(4_u128), 
                Uint256::from_u128(5_u128)
            ],
        };

        assert_eq!(
            Uint256::from_u128(5_u128), 
            Uint256::from_u128(u128::try_from(state.hashes.len()).unwrap())
        );

        state.clear();

        assert_eq!(0, state.hashes.len());
    }
}
