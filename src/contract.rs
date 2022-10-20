use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, NumberResponse, QueryMsg};
use crate::state::PHONEBOOK;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> 
{
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddNumber { number } => execute_add_number(deps, info, number.to_string()),
        ExecuteMsg::RemoveNumber {} => execute_remove_number(deps, info),
    }
}

fn execute_add_number(
    deps: DepsMut,
    info: MessageInfo,
    number: String,
) -> Result<Response, ContractError> {
    let sender: Addr = info.sender.clone();

    if PHONEBOOK.has(deps.storage, sender.clone()) {
        Err(ContractError::DuplicateKey)
    } else {
        match PHONEBOOK.save(deps.storage, sender, &number) {
            Ok(_) => {
                let res = Response::new()
                    .add_attribute("action", "add_number")
                    .add_attribute("sender", info.sender)
                    .add_attribute("number", number);
                Ok(res)
            }
            Err(e) => Err(ContractError::Std(e)),
        }
    }
}

fn execute_remove_number(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let sender: Addr = info.sender.clone();
    if PHONEBOOK.has(deps.storage, sender.clone()) {
        PHONEBOOK.remove(deps.storage, sender);
        let res = Response::new()
            .add_attribute("action", "remove_number")
            .add_attribute("sender", info.sender);
        Ok(res)
    } else {
        Err(ContractError::EmptyKey)
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNumber { address } => to_binary(&get_number(deps, address)?),
    }
}

fn get_number(deps: Deps, address: String) -> StdResult<NumberResponse> {
    let address = deps.api.addr_validate(&address)?;
    let number = PHONEBOOK
        .may_load(deps.storage, address)?
        .unwrap_or_default();
    Ok(NumberResponse { number })
}

#[cfg(test)]
mod tests {}