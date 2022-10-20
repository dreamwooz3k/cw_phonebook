use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddNumber { number: String },
    RemoveNumber,
}

#[cw_serde]
pub struct NumberResponse {
    pub number: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NumberResponse)]
    GetNumber { address: String },
}
