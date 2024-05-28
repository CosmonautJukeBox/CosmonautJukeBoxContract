use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    #[serde(rename = "change_leds")]
    ChangeLEDs { red: u8, green: u8,blue: u8 },
    #[serde(rename = "change_led_hash")]
    ChangeLEDHash { hash: Vec<u32>},
    #[serde(rename = "reset_leds")]
    ResetLEDs,
    PlaySound { index: usize }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LEDResponse)]
    #[serde(rename = "get_leds")]
    GetLEDs {},
    #[returns(HashListResponse)]
    #[serde(rename = "get_hash_list")]
    GetHashList {}
}


#[cw_serde]
#[serde(rename = "led_response")]
pub struct LEDResponse {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[cw_serde]
#[serde(rename = "hash_list_response")]
pub struct HashListResponse {
    pub hashes: Vec<u32>
}