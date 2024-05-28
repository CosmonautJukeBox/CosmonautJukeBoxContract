#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::HASH_LIST;
use crate::msg::HashListResponse;
use cosmwasm_std::to_json_binary;



/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmonaut-juke-box";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ChangeLEDs { red, green,blue } => unimplemented!(),
        ExecuteMsg::ChangeLEDHash { hash } => execute_change_led_hash(deps, env, info, hash),
        ExecuteMsg::ResetLEDs => unimplemented!(),
        ExecuteMsg::PlaySound { index } => unimplemented!()
    }

}

pub fn execute_change_led_hash(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    hash: Vec<u32>,
) -> Result<Response, ContractError> {
    let mut cfg = HASH_LIST.load(deps.storage)?;
    cfg.hashes=hash;


    let res = Response::new().add_attribute("action", "change_led_hash");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetLEDs {} => unimplemented!(),
        QueryMsg::GetHashList {} => to_json_binary(&query_hash_list(deps)?),
    }
}

pub fn query_hash_list(deps: Deps) -> StdResult<HashListResponse> {
    let cfg = HASH_LIST.load(deps.storage)?;
    Ok(HashListResponse {
        hashes: cfg.hashes.into_iter().map(|a| a.into()).collect(),
    })
}


#[cfg(test)]
mod tests {}
