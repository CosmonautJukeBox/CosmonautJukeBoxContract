use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    #[serde(rename = "change_leds")]
    ChangeLEDs { red: u8, green: u8,blue: u8 },
    #[serde(rename = "reset_leds")]
    ResetLEDs,
    PlaySound { index: usize }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LEDResponse)]
    #[serde(rename = "get_leds")]
    GetLEDs {}
}

#[cw_serde]
#[serde(rename = "led_response")]
pub struct LEDResponse {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}
