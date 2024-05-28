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
        ExecuteMsg::ResetLEDs => execute_reset_leds(deps, env, info),
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
    HASH_LIST.save(deps.storage, &cfg)?;
    let res = Response::new().add_attribute("action", "change_led_hash");
    Ok(res)
}

pub fn execute_reset_leds(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut cfg = HASH_LIST.load(deps.storage)?;
    cfg.hashes.clear();
    HASH_LIST.save(deps.storage, &cfg)?;
    let res = Response::new().add_attribute("action", "reset_leds");
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
mod tests {

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::StdError;
    use crate::msg::HashListResponse;
    use crate::state::HashList;

    #[test]
    fn test_query_hash_list() {
        let mut deps = mock_dependencies();

        let hash_list = HashList {
            hashes: vec![1, 2, 3, 4, 5],
        };
        HASH_LIST.save(&mut deps.storage, &hash_list).unwrap();

        let res = query_hash_list(deps.as_ref()).unwrap();
        assert_eq!(res, HashListResponse {
            hashes: vec![1, 2, 3, 4, 5],
        });
    }

    #[test]
    fn test_execute_change_led_hash() {
        let mut deps = mock_dependencies();

        let hash_list = HashList {
            hashes: vec![1, 2, 3, 4, 5],
        };
        HASH_LIST.save(&mut deps.storage, &hash_list).unwrap();

        let info = mock_info("creator", &[]);
        let hash = vec![1, 2, 3, 4, 5, 6];
        let msg = ExecuteMsg::ChangeLEDHash { hash: hash.clone() };
        let res = execute_change_led_hash(deps.as_mut(), mock_env(), info, hash).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "change_led_hash");

        let hash_list = HashList {
            hashes: vec![1, 2, 3, 4, 5, 6],
        };
        let res = HASH_LIST.load(deps.as_ref().storage).unwrap();
        assert_eq!(res, hash_list);
    }

    #[test]
    fn test_execute_reset_leds() {
        let mut deps = mock_dependencies();

        let hash_list = HashList {
            hashes: vec![1, 2, 3, 4, 5],
        };
        HASH_LIST.save(&mut deps.storage, &hash_list).unwrap();

        let info = mock_info("creator", &[]);
        let msg = ExecuteMsg::ResetLEDs {};
        let res = execute_reset_leds(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "reset_leds");

        let hash_list = HashList {
            hashes: vec![],
        };
        let res = HASH_LIST.load(deps.as_ref().storage).unwrap();
        assert_eq!(res, hash_list);
    }

}
