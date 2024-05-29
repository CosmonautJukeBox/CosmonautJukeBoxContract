use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    #[serde(rename = "add_led_hash")]
    AddLEDHash { hash: u32 },
    #[serde(rename = "clear_queue")]
    ClearQueue
}

#[cw_serde]
#[derive(QueryResponses)]
#[derive(cw_orch::QueryFns)]
pub enum QueryMsg {
    #[returns(HashListResponse)]
    #[serde(rename = "get_hash_list")]
    GetHashList {}
}

#[cw_serde]
#[serde(rename = "hash_list_response")]
pub struct HashListResponse {
    pub hashes: Vec<u32>
}
