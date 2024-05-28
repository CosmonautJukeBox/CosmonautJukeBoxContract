use cw_orch::{interface, prelude::*};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub const CONTRACT_ID: &str = "cosmonaut_juke_box";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty, id = CONTRACT_ID)]
pub struct CosmonautContract;

impl<Chain> Uploadable for CosmonautContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cosmonaut_juke_box")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
                //.with_migrate(crate::contract::migrate),
        )
    }
}
