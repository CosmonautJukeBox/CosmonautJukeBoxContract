use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint256;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    #[serde(rename = "add_led_hash")]
    AddLEDHash { hash: Uint256 },
    #[serde(rename = "remove_hash")]
    RemoveHash,
    #[serde(rename = "clear_queue")]
    ClearQueue,
}

#[cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(HashListResponse)]
    #[serde(rename = "get_hash_list")]
    GetHashList,
}

#[cw_serde]
#[serde(rename = "hash_list_response")]
pub struct HashListResponse {
    pub hashes: Vec<Uint256>,
}
