use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct HashList {
    pub hashes: Vec<u32>,
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
            hashes: vec![1, 2, 3, 4, 5],
        };

        assert_eq!(5, state.hashes.len());

        state.clear();

        assert_eq!(0, state.hashes.len());
    }
}
