use cosmwasm_std::StdError;
use thiserror::Error;
use pix0_contract_common::error::CommonContractError;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("CustomErrorMesg")]
    CustomErrorMesg { message : String },

    
    #[error("ContractInfoNotFound")]
    ContractInfoNotFound { message : String },

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}


impl From<CommonContractError> for ContractError {
    fn from(error : CommonContractError) -> ContractError {
        
        match error {

            CommonContractError::ContractInfoNotFound { message } => 
            ContractError::ContractInfoNotFound { message: message }
            ,

            _ => ContractError::CustomErrorMesg { message: "Custom error".to_string() }

        }
    }
}
