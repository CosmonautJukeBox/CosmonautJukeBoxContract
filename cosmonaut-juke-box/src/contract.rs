#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{HASH_LIST, HashList};
use crate::msg::HashListResponse;
use cosmwasm_std::to_json_binary;



/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmonaut-juke-box";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new().add_attribute("action", "instantiate");
    let hash_list = HashList {
        hashes: vec![],
    };
    HASH_LIST.save(deps.storage, &hash_list)?;
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddLEDHash { hash } => execute_add_led_hash(deps, env, info, hash),
        ExecuteMsg::ClearQueue => execute_clear_queue(deps, env, info),
        ExecuteMsg::RemoveHash => execute_remove_hash(deps, env, info)
    }

}

pub fn execute_add_led_hash(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    hash: u32,
) -> Result<Response, ContractError> {
    const MAX_LENGTH: usize = 10;

    let mut cfg = HASH_LIST.load(deps.storage)?;

    cfg.hashes.push(hash);
    if cfg.hashes.len() > MAX_LENGTH {
        cfg.hashes.remove(0);
    }

    HASH_LIST.save(deps.storage, &cfg)?;
    let res = Response::new().add_attribute("action", "add_led_hash");
    Ok(res)
}

pub fn execute_clear_queue(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut cfg = HASH_LIST.load(deps.storage)?;
    cfg.hashes.clear();
    HASH_LIST.save(deps.storage, &cfg)?;
    let res = Response::new().add_attribute("action", "clear_queue");
    Ok(res)
}

pub fn execute_remove_hash(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut cfg = HASH_LIST.load(deps.storage)?;
    cfg.hashes.remove(0);
    HASH_LIST.save(deps.storage, &cfg)?;
    let res = Response::new().add_attribute("action", "remove_hash");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
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
        let hash = 6;

        let res = execute_add_led_hash(deps.as_mut(), mock_env(), info, hash).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "add_led_hash");

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

        let res = execute_clear_queue(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "clear_queue");

        let hash_list = HashList {
            hashes: vec![],
        };
        let res = HASH_LIST.load(deps.as_ref().storage).unwrap();
        assert_eq!(res, hash_list);
    }

    #[test]
    fn test_execute_remove_hash() {
        let mut deps = mock_dependencies();

        let hash_list = HashList {
            hashes: vec![1, 2, 3, 4, 5],
        };
        HASH_LIST.save(&mut deps.storage, &hash_list).unwrap();

        let info = mock_info("creator", &[]);

        let res = execute_remove_hash(deps.as_mut(), mock_env(), info).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "remove_hash");

        let hash_list = HashList {
            hashes: vec![2, 3, 4, 5],
        };
        let res = HASH_LIST.load(deps.as_ref().storage).unwrap();
        assert_eq!(res, hash_list);
    }

}
