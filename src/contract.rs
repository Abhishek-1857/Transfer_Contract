use crate::msg::{QueryMsg,InstantiateMsg,ExecuteMsg};
use cosmwasm_std::{
  Binary, Deps, DepsMut, Env, MessageInfo, Response,Storage, to_vec, Addr
};
use cosmwasm_storage::{PrefixedStorage};
use crate::state::{Constants,ADMIN,MINT_REQUEST_MAPPING,SIGNERS, MintRequestInfo,PROPOSAL_ID,HAS_APPROVED};
use crate::error::ContractError;

pub const PREFIX_CONFIG: &[u8] = b"config";
pub const PREFIX_BALANCES: &[u8] = b"balances";
pub const KEY_CONSTANTS: &[u8] = b"constants";
pub const KEY_TOTAL_SUPPLY: &[u8] = b"total_supply";
pub const PREFIX_FREEZE_AMOUNT: &[u8]= b"freeze_amount";




pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response,ContractError> {
    let proposal_id:u128=0;
    PROPOSAL_ID.save(deps.storage, &proposal_id)?;
    let mut config_store = PrefixedStorage::new(deps.storage, PREFIX_CONFIG);
    let constants = to_vec(&Constants {
        name: msg.name,
    })?;
    //admin info
    let admin=_info.sender;
    config_store.set(KEY_CONSTANTS, &constants);
    ADMIN.save(deps.storage, &admin)?;
    let mut signers:Vec<Addr>=Vec::new();
    for signer in msg.signers{
        signers.push(signer.clone());
        HAS_APPROVED.save(deps.storage, (signer,proposal_id),&false)?;
    }
    SIGNERS.save(deps.storage, &signers)?;
    Ok(Response::default())
}

pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary,ContractError> {
    todo!()
}


pub fn execute(deps:DepsMut,_env:Env,info:MessageInfo,msg:ExecuteMsg) -> Result<Response,ContractError>{
    use ExecuteMsg::*;
    match msg {
        CreateMintRequest {  }=>execute::try_create_mint_request(deps, info),
        ApproveMintRequest { proposal_id }=>execute::try_approve_mint_request(deps, info, proposal_id),
        Mint { proposal_id}=>execute::try_mint(deps, info, proposal_id)
    }
}
mod query {
    // pub fn bytes_to_u128(data: &[u8]) -> Result<u128, ContractError> {
    //     match data[0..16].try_into() {
    //         Ok(bytes) => Ok(u128::from_be_bytes(bytes)),
    //         Err(_) => Err(ContractError::CorruptedDataFound {}),
    //     }
    // }

    // pub fn read_u128(store: &ReadonlyPrefixedStorage, key: &Addr) -> Result<u128, ContractError> {
    //     let result = store.get(key.as_str().as_bytes());
    //     match result {
    //         Some(data) => bytes_to_u128(&data),
    //         None => Ok(0u128),
    //     }
    // }
    
    // fn read_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
    //     let balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_BALANCES);
    //     read_u128(&balance_store, owner)
    // }

    // fn read_frozen_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
    //     let frozen_balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_FREEZE_AMOUNT);
    //     read_u128(&frozen_balance_store, owner)
    // }
}
mod execute{
    use super::*;
   
pub fn try_create_mint_request(deps:DepsMut,info:MessageInfo)->Result<Response,ContractError>{
let mut  propsal_id=PROPOSAL_ID.load(deps.storage)?;
let propsal_req=MintRequestInfo{
    proposal_id:propsal_id,
    owner:info.sender,
    approvals:0,
    completed:false
};
MINT_REQUEST_MAPPING.save(deps.storage, propsal_id, &propsal_req)?;
propsal_id+=1;
PROPOSAL_ID.save(deps.storage,&propsal_id)?;
Ok(Response::new()
.add_attribute("action", "create_mint_req"))
}

pub fn try_approve_mint_request(deps:DepsMut,info:MessageInfo,proposal_id:u128)-> Result<Response,ContractError>{
let signers=SIGNERS.load(deps.storage)?;
if !signers.contains(&info.sender){
    return Err(ContractError::Unauthorised { address: info.sender });
    }
let mut has_approved=HAS_APPROVED.load(deps.storage,(info.sender.clone(),proposal_id)).unwrap_or(false);
if has_approved{
    return Err(ContractError::AlreadyApproved { address: info.sender });
}
has_approved=true;
let mut req_info=MINT_REQUEST_MAPPING.load(deps.storage, proposal_id)?;
if !req_info.proposal_id==proposal_id{
    return Err(ContractError::InvalidProposalId { proposal_id });
}
req_info.approvals+=1;
MINT_REQUEST_MAPPING.save(deps.storage, proposal_id, &req_info)?;
HAS_APPROVED.save(deps.storage, (info.sender,proposal_id), &has_approved)?;
Ok(Response::new().add_attribute("action", "approve_mint_requestr"))
}

pub fn try_mint(deps: DepsMut,info:MessageInfo,proposal_id:u128)->Result<Response,ContractError>{
    let mut  req_info=MINT_REQUEST_MAPPING.load(deps.storage, proposal_id)?;
    if req_info.owner!=info.sender{
        return Err(ContractError::NotOwner {  });
    }
    // let signers=SIGNERS.load(deps.storage)?;
    // if !req_info.approvals>=(signers.len()/2).try_into().unwrap(){
    //  return Err(ContractError::NotEnoughApproval { approvals: req_info.approvals });
    // }
    if req_info.completed{
        return Err(ContractError::Completed { proposal_id });
    }
    req_info.completed=true;
    MINT_REQUEST_MAPPING.save(deps.storage, proposal_id, &req_info)?;
    Ok(Response::new().add_attribute("action", "mint"))
}
    
}




// #[cfg(test)]
// mod tests{
//     use super::*;
//     use crate::msg::InitialBalance;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{from_slice, Addr, Env, MessageInfo, Storage, Timestamp, Uint128};
//     use cosmwasm_storage::ReadonlyPrefixedStorage;

//     fn mock_env_height(signer: &str, height: u64, time: u64) -> (Env, MessageInfo) {
//         let mut env = mock_env();
//         let info = mock_info(signer, &[]);
//         env.block.height = height;
//         env.block.time = Timestamp::from_seconds(time);
//         (env, info)
//     }

//     fn get_constants(storage: &dyn Storage) -> Constants {
//         let config_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_CONFIG);
//         let data = config_storage
//             .get(KEY_CONSTANTS)
//             .expect("no config data stored");
//         from_slice(&data).expect("invalid data")
//     }

//     fn get_total_supply(storage: &dyn Storage) -> u128 {
//         let config_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_CONFIG);
//         let data = config_storage
//             .get(KEY_TOTAL_SUPPLY)
//             .expect("no decimals data stored");
//         return execute::bytes_to_u128(&data).unwrap();
//     }

//     fn get_balance(storage: &dyn Storage, address: &Addr) -> u128 {
//         let balances_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_BALANCES);
//         return execute::read_u128(&balances_storage, address).unwrap();
//     }

//     fn get_frozen_balance(storage: &dyn Storage, address: &Addr) -> u128 {
//         let frozen_balances_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_FREEZE_AMOUNT);
//         return execute::read_u128(&frozen_balances_storage, address).unwrap();
//     }

//     fn get_shareholder(storage: &dyn Storage) -> Result<Vec<String>,ContractError> {
//         let shareholder =   SHAREHOLDERS.load(storage)?;
//         return Ok(execute::get_shareholder(&shareholder));
//     }


//     mod instantiate {
//         use super::*;
//         #[test]
//         fn works() {
//             let mut deps = mock_dependencies();
//             let instantiate_msg = InstantiateMsg {
//                 name: "Provenance Token".to_string(),
//                 symbol: "PRV".to_string(),
//                 max_supply:1000000,
//                 initial_balances: [InitialBalance {
//                     address: "creator".to_string(),
//                     amount: Uint128::from(11223344u128),
//                     freeze_amount:Uint128::from(100u128)
//                 }]
//                 .to_vec(),
//                 share_holders: ["creator".to_string()].to_vec(),
//                 authorised_countries:[91].to_vec(),
//                 max_hold_balance: 10000,
//             };
//             let (env, info) = mock_env_height("creator", 450, 550);
//             let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
//             assert_eq!(0, res.messages.len());
//             assert_eq!(
//                 get_constants(&deps.storage),
//                 Constants {
//                     name: "Provenance Token".to_string(),
//                     symbol: "PRV".to_string(),
//                     max_supply: 1000000,
//                 }
//             );
//             assert_eq!(
//                 get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//                 11223344
//             );
//             assert_eq!(get_total_supply(&deps.storage), 11223344);
//             assert_eq!(get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),100);
//             assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string()].to_vec()));
//         }

// }

// mod transfer {
//     use super::*;
//     use cosmwasm_std::Event;

//     fn make_instantiate_msg() -> InstantiateMsg {
//         InstantiateMsg {
//             name: "Provenance Token".to_string(),
//             symbol: "PRV".to_string(),
//             max_supply:1000000,
//             initial_balances: [InitialBalance {
//                 address: "creator".to_string(),
//                 amount: Uint128::from(11223344u128),
//                 freeze_amount:Uint128::from(100u128)
//             }]
//             .to_vec(),
//             share_holders: ["creator".to_string(),"creator2".to_string()].to_vec(),
//             authorised_countries:[91].to_vec(),
//             max_hold_balance: 10000,
//         }
//     }

//     #[test]
//     fn transfer_to_address() {
//         let mut deps = mock_dependencies();
//         let instantiate_msg = make_instantiate_msg();
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
//         assert_eq!(0, res.messages.len());
//         // Initial state
//         assert_eq!(
//             get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             11223344
//         );
//         assert_eq!(
//             get_balance(&deps.storage, &Addr::unchecked("addr1111".to_string())),
//             0
//         );
      
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//         // Transfer
//         let transfer_msg = ExecuteMsg::Transfer {
//             reciever: "addr1111".to_string(),
//             amount:Uint128::from(1u128),
//             countrycode: 91,
//         };
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let transfer_result = execute(deps.as_mut(), env, info, transfer_msg).unwrap();
//         assert_eq!(transfer_result.messages.len(), 0);
//         let expected_event = Event::new("transfer")
//         .add_attribute("action","transfer");
        

//     // Verify the response
//     assert_eq!(
//         transfer_result,
//         Response::new()
//             .add_event(expected_event.clone())
//     );

//     // Verify the emitted event
//     let events = transfer_result.events.clone();
//     assert_eq!(1, events.len()); // Ensure there is only one event emitted
//     assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
//         // New state
//         assert_eq!(
//             get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             11223343
//         ); // -1
//         assert_eq!(
//             get_balance(&deps.storage, &Addr::unchecked("addr1111".to_string())),
//             1
//         ); // +1
        
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//     }



//     #[test] 
//     fn freeze_token() {
//         let mut deps = mock_dependencies();
//         let instantiate_msg = make_instantiate_msg();
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
//         assert_eq!(0, res.messages.len());
//         // Initial state
//         assert_eq!(
//             get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             100
//         );
        
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//         // Transfer
//         let freeze_token_message = ExecuteMsg::FreezeToken {
//             amount:Uint128::from(1u128),
//         };
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let freeze_token_result = execute(deps.as_mut(), env, info, freeze_token_message).unwrap();
//         assert_eq!(freeze_token_result.messages.len(), 0);
//         let expected_event = Event::new("freeze_amount")
//         .add_attribute("action","freeze_amount");
        

//     // Verify the response
//     assert_eq!(
//         freeze_token_result,
//         Response::new()
//             .add_event(expected_event.clone())
//     );

//     // Verify the emitted event
//     let events = freeze_token_result.events.clone();
//     assert_eq!(1, events.len()); // Ensure there is only one event emitted
//     assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
//         // New state
//         assert_eq!(
//             get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             101
//         ); // +1
       
        
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//     }

//     #[test] 
//     fn un_freeze_token() {
//         let mut deps = mock_dependencies();
//         let instantiate_msg = make_instantiate_msg();
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
//         assert_eq!(0, res.messages.len());
//         // Initial state
//         assert_eq!(
//             get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             100
//         );
        
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//         // Transfer
//         let unfreeze_token_message = ExecuteMsg::UnfreezeToken{
//             amount:Uint128::from(1u128),
//         };
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let unfreeze_token_result = execute(deps.as_mut(), env, info, unfreeze_token_message).unwrap();
//         assert_eq!(unfreeze_token_result.messages.len(), 0);
//         let expected_event = Event::new("unfreeze_amount")
//         .add_attribute("action","unfreeze_amount");
        

//     // Verify the response
//     assert_eq!(
//         unfreeze_token_result,
//         Response::new()
//             .add_event(expected_event.clone())
//     );

//     // Verify the emitted event
//     let events = unfreeze_token_result.events.clone();
//     assert_eq!(1, events.len()); // Ensure there is only one event emitted
//     assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
//         // New state
//         assert_eq!(
//             get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
//             99
//         ); // -1
       
        
//         assert_eq!(get_total_supply(&deps.storage), 11223344);
//     }

//     #[test] 
//     fn remove_shareholder() {
//         let mut deps = mock_dependencies();
//         let instantiate_msg = make_instantiate_msg();
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
//         assert_eq!(0, res.messages.len());
//         // Initial state
//         assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string(),"creator2".to_string()].to_vec()));
        
//         // Transfer
//         let remove_shareholder_message = ExecuteMsg::RemoveShareholder{
//            account:"creator2".to_string(),
//         };
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let remove_shareholder_result = execute(deps.as_mut(), env, info, remove_shareholder_message).unwrap();
//         assert_eq!(remove_shareholder_result.messages.len(), 0);
//     //     let expected_event = Event::new("remove_shareholder")
//     //     .add_attribute("action","remove_shareholder");
//     // // Verify the response
//     // assert_eq!(
//     //     remove_shareholder_result,
//     //     Response::new()
//     //         .add_event(expected_event.clone())
//     // );
//     // // Verify the emitted event
//     // let events = remove_shareholder_result.events.clone();
//     // assert_eq!(1, events.len()); // Ensure there is only one event emitted
//     // assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
//         // New state
//         assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string()].to_vec()));
//     }  
// }
// mod query {
//     use super::*;
//     use cosmwasm_std::{attr, Addr};


//     fn make_instantiate_msg() -> InstantiateMsg {
//         InstantiateMsg {
//             name: "Provenance Token".to_string(),
//             symbol: "PRV".to_string(),
//             max_supply:1000000,
//             initial_balances: [InitialBalance {
//                 address: "creator".to_string(),
//                 amount: Uint128::from(11223344u128),
//                 freeze_amount:Uint128::from(100u128)
//             }]
//             .to_vec(),
//             share_holders: ["creator".to_string(),"creator2".to_string()].to_vec(),
//             authorised_countries:[91].to_vec(),
//             max_hold_balance: 10000,
//         }
//     }

//     #[test]
//     fn can_query_balance_of_existing_address() {
//         let mut deps = mock_dependencies();
//         let instantiate_msg = make_instantiate_msg();
//         let (env, info) = mock_env_height("creator", 450, 550);
//         let res = instantiate(deps.as_mut(), env.clone(), info, instantiate_msg).unwrap();
//         assert_eq!(0, res.messages.len());
//         let query_msg = QueryMsg::Balance {
//             address: "creator".to_string(),
//         };
//         let query_result = query(deps.as_ref(), env, query_msg).unwrap();
//         assert_eq!(query_result.as_slice(), b"{\"balance\":\"11223344\"}");
//     }

// }
// }