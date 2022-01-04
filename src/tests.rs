#![cfg(test)]
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
// use cosmwasm_std::{Addr, coins, from_binary, to_binary, CosmosMsg, DepsMut, Response, WasmMsg, Uint128, Uint256};
use cosmwasm_std::{Addr, coins, from_binary, to_binary, CosmosMsg, DepsMut, Response, WasmMsg, Uint128};


    // use cosmwasm_std::{coins, from_binary, Addr};

use cw721::{
    ApprovedForAllResponse, ContractInfoResponse, Cw721Query, Cw721ReceiveMsg, Expiration,
    OwnerOfResponse,
};

use crate::{
    ContractError, PepperContract, ExecuteMsg, InstantiateMsg, MintMsg, MintTagMsg, QueryMsg, Metadata, CountResponse, BalanceResponse, PriceResponse, PublicKeyResponse, TagsResponse,
};
use crate::local_cw721_base::query::TokensResponse;

use crate::entry::{execute,query,instantiate};

// user create::msg:MintMsg;

const MINTER: &str = "merlin";
const CONTRACT_NAME: &str = "Magic Power";
const SYMBOL: &str = "MGK";

fn setup_contract(deps: DepsMut<'_>) -> PepperContract<'static> {
    let contract = PepperContract::<>::default();
    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
    };
    let info = mock_info("creator", &[]);
    let res = instantiate(deps, mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());
    contract
}

#[test]
fn proper_instantiation() {
    let mut deps = mock_dependencies(&[]);
    let contract = PepperContract::<>::default();

    let msg = InstantiateMsg {
        name: CONTRACT_NAME.to_string(),
        symbol: SYMBOL.to_string(),
        minter: String::from(MINTER),
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract
        .instantiate(deps.as_mut(), mock_env(), info, msg)
        .unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(MINTER, res.minter);
    let info = contract.contract_info(deps.as_ref()).unwrap();
    assert_eq!(
        info,
        ContractInfoResponse {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
        }
    );

    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(0, tokens.tokens.len());
}

#[test]
fn proper_extended_initialization() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // it worked, let's query the state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    let value: CountResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.count);
}

// #[test]
// fn extended_functions() {
//     let mut deps = mock_dependencies(&[]);

//     setup_contract(deps.as_mut());

//     // count is 0
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//     let value: CountResponse = from_binary(&res).unwrap();
//     assert_eq!(0, value.count);

//     let media_addr = Addr::unchecked("terra1333veey879eeqcff8j3gfcgwt8cfrg9mq20v6f");


//     // can't ask for the key for free
//     let unauth_info = mock_info("anyone", &coins(1u128, "uluna"));
//     let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
//     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//     match res {
//         Err(ContractError::NotEnoughFunds {}) => {}
//         _ => panic!("Must return NotEnoughFunds error"),
//     }

//     // can't ask for the key for free (no coins at all)
//     let unauth_info = mock_info("anyone", &[]);
//     let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
//     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//     match res {
//         Err(ContractError::NotEnoughFunds {}) => {}
//         _ => panic!("Must return NotEnoughFunds error"),
//     }

//     // anyone can ask for the key
//     let info = mock_info("anyone", &coins(1000000u128, "uluna"));
//     let watcher = info.sender.clone();

//     let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//     // server can query the key
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetKey { media: media_addr.clone(), addr: watcher.clone() }).unwrap();
//     let value: KeyResponse = from_binary(&res).unwrap();
//     assert_eq!("key", value.key);
//     assert_eq!(true, value.is_public);

//     // other users can't upload encoded key
//     let unauth_info = mock_info("anyone", &coins(2, "luna"));
//     let msg = ExecuteMsg::FillKey { media: media_addr.clone(), addr: watcher.clone(), key: "filledkey".to_string() };
//     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//     match res {
//         Err(ContractError::Unauthorized {}) => {}
//         _ => panic!("Must return unauthorized error"),
//     }

//     // server can upload encoded key
//     let msg = ExecuteMsg::FillKey { media: media_addr.clone(), addr: watcher.clone(), key: "filledkey".to_string() };
//     let info = mock_info("creator", &coins(2, "luna"));
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//     // anyone can query the key
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetKey { media: media_addr.clone(), addr: watcher.clone() }).unwrap();
//     let value: KeyResponse = from_binary(&res).unwrap();
//     assert_eq!("filledkey", value.key);
//     assert_eq!(false, value.is_public);




//     // minter balance should be increased
//     let info = mock_info(&MINTER.to_string(), &coins(2, "token"));
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
//     let value: BalanceResponse = from_binary(&res).unwrap();
//     assert_eq!("1000000", value.uluna); // 80% of the price as owner. 10% of the price as creator



//     // minter can withdraw uluna
//     let msg = ExecuteMsg::Withdraw {  };
//     let info = mock_info(&MINTER.to_string(), &[]);
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();



//     // minter balance should be 0 now
//     let info = mock_info(&MINTER.to_string(),  &[]);
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
//     let value: BalanceResponse = from_binary(&res).unwrap();
//     assert_eq!("0", value.uluna); // 80% of the price as owner. 10% of the price as creator

















//     // asking the same code again should change nothing
//     let info = mock_info("anyone", &coins(2, "uluna"));
//     let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

//     // key is still set to encoded
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetKey { media: media_addr.clone(), addr: watcher.clone() }).unwrap();
//     let value: KeyResponse = from_binary(&res).unwrap();
//     assert_eq!("filledkey", value.key);
//     assert_eq!(false, value.is_public);


//     // uploading another code changes nothing
//     let msg = ExecuteMsg::FillKey { media: media_addr.clone(), addr: watcher.clone(), key: "fille3333dkey".to_string() };
//     let info = mock_info("creator", &coins(2, "luna"));
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


//     // key is still set to encoded
//     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetKey { media: media_addr.clone(), addr: watcher.clone() }).unwrap();
//     let value: KeyResponse = from_binary(&res).unwrap();
//     assert_eq!("filledkey", value.key);
//     assert_eq!(false, value.is_public);

//     // asking the code by another user
//     let info = mock_info("somebody", &coins(1000000u128, "uluna"));
//     let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
//     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


// }

#[test]
fn minting() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    // Can!

    // random cannot mint
    // let random = mock_info("random", &[]);
    // let err = execute(deps.as_mut(), mock_env(), random, mint_msg.clone())
    //     .unwrap_err();
    // assert_eq!(err, ContractError::Unauthorized {});

    // Can!


    // minter can mint
    let allowed = mock_info(MINTER, &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(1, count.count);

    // unknown nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "unknown".to_string())
        .unwrap_err();

    // this nft info is correct
    let info = contract.nft_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info.token_uri,  Some(token_uri)
    );

    // owner info is correct
    let owner = contract
        .owner_of(deps.as_ref(), mock_env(), token_id.clone(), true)
        .unwrap();
    assert_eq!(
        owner,
        OwnerOfResponse {
            owner: String::from("medusa"),
            approvals: vec![],
        }
    );

    // Cannot mint same token_id again
    let mint_msg2 = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("hercules"),
        token_uri: None,
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let allowed = mock_info(MINTER, &[]);
    let err = execute(deps.as_mut(), mock_env(), allowed, mint_msg2)
        .unwrap_err();
    assert_eq!(err, ContractError::Claimed {});

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id], tokens.tokens);
}






#[test]
fn mint_and_buy() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let token_id = "somenfttobuy".to_string();
    let token_addr = Addr::unchecked("somenfttobuy");
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("medusa"),
        token_uri: Some(token_uri.clone()),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    // minter can mint
    let allowed = mock_info(MINTER, &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg)
        .unwrap();

    // ensure num tokens increases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(1, count.count);

    // this nft info is correct
    let info = contract.nft_info(deps.as_ref(), token_id.clone()).unwrap();
    assert_eq!(
        info.token_uri,  Some(token_uri)
    );

    // it worked, let's query the price
    let media_addr = Addr::unchecked(token_id.clone());
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice { media: media_addr }).unwrap();
    let value: PriceResponse = from_binary(&res).unwrap();
    assert_eq!("1000000", value.uluna); // default NFT watch price = 1 Luna

    // anyone can ask for the key
    let info = mock_info("anyone", &coins(2000000u128, "uluna"));
    // let watcher = info.sender.clone();
    let msg = ExecuteMsg::AskForKey { media: token_addr.clone(), key: "key".to_string() };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

}


#[test]
fn set_the_price_with_mint() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    let token_id = "som222enfttobuy".to_string();
    let token_addr = Addr::unchecked("som222enfttobuy");
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("whoever"),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(100000u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    // whoever can mint
    let allowed = mock_info("whoever", &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg).unwrap();

    // it worked, let's query the price
    let media_addr = Addr::unchecked(token_id.clone());
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPrice { media: media_addr.clone() }).unwrap();
    let value: PriceResponse = from_binary(&res).unwrap();
    assert_eq!("100000", value.uluna); // Price is 0.1 luna now


    // can't ask for the key if not enough luna
    let unauth_info = mock_info("anyone", &coins(99999u128, "uluna"));
    let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
    let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    match res {
        Err(ContractError::NotEnoughFunds {}) => {}
        _ => panic!("Must return NotEnoughFunds error"),
    }


    // anyone can ask for the key with enough luna
    let info = mock_info("anyone", &coins(100000u128, "uluna"));
    // let watcher = info.sender.clone();
    let msg = ExecuteMsg::AskForKey { media: token_addr.clone(), key: "key".to_string() };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


    // minter balance should be increased
    let info = mock_info("whoever", &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("90000", value.uluna); // 80% of the price as owner. 10% of the price as creator


    // contact creator balance should be increased
    let info = mock_info(MINTER, &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("10000", value.uluna); // 80% of the price as owner. 10% of the price as creator


    // others can't change the price
    let allowed = mock_info("somebodyotherthanowner", &[]);
    let msg = ExecuteMsg::SetPrice { media: media_addr.clone(), price: Uint128::from(50000u128) };
    let res = execute(deps.as_mut(), mock_env(), allowed, msg);
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Must return Unauthorized error"),
    };

    // owner can't set value to wrong value
    let allowed = mock_info("whoever", &[]);
    let msg = ExecuteMsg::SetPrice { media: media_addr.clone(), price: Uint128::from(0u128) };
    let res = execute(deps.as_mut(), mock_env(), allowed, msg);
    match res {
        Err(ContractError::Invalid {}) => {}
        _ => panic!("Must return Invalid error"),
    };

    // owner can change the price, to 0.2 Luna
    let allowed = mock_info("whoever", &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, ExecuteMsg::SetPrice { media: media_addr.clone(), price: Uint128::from(200000u128) }).unwrap();

    // can't ask for the key if not enough luna
    let unauth_info = mock_info("anyonenew", &coins(199999u128, "uluna"));
    let msg = ExecuteMsg::AskForKey { media: media_addr.clone(), key: "key".to_string() };
    let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    match res {
        Err(ContractError::NotEnoughFunds {}) => {}
        _ => panic!("Must return NotEnoughFunds error"),
    }

    // anyone can ask for the key with enough luna
    let info = mock_info("anyonenew", &coins(200000u128, "uluna"));
    // let watcher = info.sender.clone();
    let msg = ExecuteMsg::AskForKey { media: token_addr.clone(), key: "key".to_string() };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // minter balance should be increased
    let info = mock_info("whoever", &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("270000", value.uluna); // 0.09+0.18


    // contact creator balance should be increased
    let info = mock_info(MINTER, &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("30000", value.uluna); // 0.01 + 0.02

}



#[test]
fn minimum_price_handling() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // somebody can't set the minimum watch price on a contract
    let allowed = mock_info("somebodyotherthanowner", &[]);
    let msg = ExecuteMsg::SetMinimumPrice { price: Uint128::from(50000u128) };
    let res = execute(deps.as_mut(), mock_env(), allowed, msg);
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Must return Unauthorized error"),
    };

    // contract owner can set the minimum price
    let allowed = mock_info(MINTER, &[]);
    let msg = ExecuteMsg::SetMinimumPrice { price: Uint128::from(50000u128) };
    let _res = execute(deps.as_mut(), mock_env(), allowed, msg).unwrap();


    let token_id = "token_id".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let nft_minter = "whoever";

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from(nft_minter),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(1u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    // can't mint setting watch price to lower then default minimum
    let allowed = mock_info(nft_minter, &[]);
    let res = execute(deps.as_mut(), mock_env(), allowed, mint_msg);
    match res {
        Err(ContractError::Invalid {}) => {}
        _ => panic!("Must return Invalid error"),
    };

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from(nft_minter),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(49999u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    // can't mint setting watch price to lower then default minimum
    let allowed = mock_info(nft_minter, &[]);
    let res = execute(deps.as_mut(), mock_env(), allowed, mint_msg);
    match res {
        Err(ContractError::Invalid {}) => {}
        _ => panic!("Must return Invalid error"),
    };

    // can mint setting the price to minimum
    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from(nft_minter),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(50000u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });
    let allowed = mock_info(nft_minter, &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg).unwrap();


    // can't update watch price to lower than minimum
    let allowed = mock_info(nft_minter, &[]);
    let msg = ExecuteMsg::SetPrice { media: Addr::unchecked(token_id.clone()), price: Uint128::from(49000u128) };
    let res = execute(deps.as_mut(), mock_env(), allowed, msg);
    match res {
        Err(ContractError::Invalid {}) => {}
        _ => panic!("Must return Invalid error"),
    };

    // can  update watch price above than minimum
    let allowed = mock_info(nft_minter, &[]);
    let msg = ExecuteMsg::SetPrice { media: Addr::unchecked(token_id.clone()), price: Uint128::from(159000u128) };
    let _res = execute(deps.as_mut(), mock_env(), allowed, msg).unwrap();


}






#[test]
fn use_metadata_extension() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let info = mock_info(MINTER, &[]);
    let init_msg = InstantiateMsg {
        name: "SpaceShips".to_string(),
        symbol: "SPACE".to_string(),
        minter: MINTER.to_string(),
    };
    contract
        .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
        .unwrap();

    let token_id = "Enterprise";
    let mint_msg = MintMsg {
        token_id: token_id.to_string(),
        owner: "john".to_string(),
        token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
        extension: Some(Metadata {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    };
    let exec_msg = ExecuteMsg::Mint(mint_msg.clone());

    execute(deps.as_mut(), mock_env(), info, exec_msg)
        .unwrap();

    let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
    assert_eq!(res.token_uri, mint_msg.token_uri);


    let extension = res.extension.unwrap_or_default();
    let mint_msg_extension = mint_msg.extension.unwrap_or_default();

    assert_eq!(extension.description, mint_msg_extension.description);
    assert_eq!(extension.name, mint_msg_extension.name);
}

#[test]
fn burning() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    let token_id = "petrify".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/petrify".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: MINTER.to_string(),
        token_uri: Some(token_uri),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let burn_msg = ExecuteMsg::Burn { token_id };

    // mint some NFT
    let allowed = mock_info(MINTER, &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed.clone(), mint_msg)
        .unwrap();

    // random not allowed to burn
    let random = mock_info("random", &[]);
    let err = execute(deps.as_mut(), mock_env(), random, burn_msg.clone())
        .unwrap_err();

    assert_eq!(err, ContractError::Unauthorized {});

    let _ = execute(deps.as_mut(), mock_env(), allowed, burn_msg)
        .unwrap();

    // ensure num tokens decreases
    let count = contract.num_tokens(deps.as_ref()).unwrap();
    assert_eq!(0, count.count);

    // trying to get nft returns error
    let _ = contract
        .nft_info(deps.as_ref(), "petrify".to_string())
        .unwrap_err();

    // list the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert!(tokens.tokens.is_empty());
}

#[test]
fn transferring_nft() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let minter = mock_info(&String::from("venus"), &[]);
    execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // random cannot transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("random"),
        token_id: token_id.clone(),
    };

    let err = execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // owner can
    let random = mock_info("venus", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("random"),
        token_id: token_id.clone(),
    };

    let res = execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "transfer_nft")
            .add_attribute("sender", "venus")
            .add_attribute("recipient", "random")
            .add_attribute("token_id", token_id.clone())
    );


    // anyone can ask for the key with enough luna
    let info = mock_info("anyonenew", &coins(1000000u128, "uluna"));
    // let watcher = info.sender.clone();
    let msg = ExecuteMsg::AskForKey { media: Addr::unchecked(token_id.clone()), key: "key".to_string() };
    let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();


    let info = mock_info(&String::from("random"), &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("800000", value.uluna); // 80% as for nft owner

    let info = mock_info(&String::from("venus"), &coins(2, "token"));
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance { addr: info.sender.clone() }).unwrap();
    let value: BalanceResponse = from_binary(&res).unwrap();
    assert_eq!("100000", value.uluna); // Only 10%, as for original nft minter
}

#[test]
fn sending_nft() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "melt".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("venus"),
        token_uri: Some(token_uri),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let minter = mock_info(MINTER, &[]);
    execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    let msg = to_binary("You now have the melting power").unwrap();
    let target = String::from("another_contract");
    let send_msg = ExecuteMsg::SendNft {
        contract: target.clone(),
        token_id: token_id.clone(),
        msg: msg.clone(),
    };

    let random = mock_info("random", &[]);
    let err = execute(deps.as_mut(), mock_env(), random, send_msg.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Unauthorized {});

    // but owner can
    let random = mock_info("venus", &[]);
    let res = execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    let payload = Cw721ReceiveMsg {
        sender: String::from("venus"),
        token_id: token_id.clone(),
        msg,
    };
    let expected = payload.into_cosmos_msg(target.clone()).unwrap();
    // ensure expected serializes as we think it should
    match &expected {
        CosmosMsg::Wasm(WasmMsg::Execute { contract_addr, .. }) => {
            assert_eq!(contract_addr, &target)
        }
        m => panic!("Unexpected message type: {:?}", m),
    }
    // and make sure this is the request sent by the contract
    assert_eq!(
        res,
        Response::new()
            .add_message(expected)
            .add_attribute("action", "send_nft")
            .add_attribute("sender", "venus")
            .add_attribute("recipient", "another_contract")
            .add_attribute("token_id", token_id)
    );
}

#[test]
fn approving_revoking() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    // Mint a token
    let token_id = "grow".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/grow".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let minter = mock_info(MINTER, &[]);
    execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // Give random transferring power
    let approve_msg = ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = execute(deps.as_mut(), mock_env(), owner, approve_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve")
            .add_attribute("sender", "demeter")
            .add_attribute("spender", "random")
            .add_attribute("token_id", token_id.clone())
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("person"),
        token_id: token_id.clone(),
    };
    execute(deps.as_mut(), mock_env(), random, transfer_msg)
        .unwrap();

    // Approvals are removed / cleared
    let query_msg = QueryMsg::OwnerOf {
        token_id: token_id.clone(),
        include_expired: None,
    };
    let res: OwnerOfResponse = from_binary(
        &query(deps.as_ref(), mock_env(), query_msg.clone())
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
            approvals: vec![],
        }
    );

    // Approve, revoke, and check for empty, to test revoke
    let approve_msg = ExecuteMsg::Approve {
        spender: String::from("random"),
        token_id: token_id.clone(),
        expires: None,
    };
    let owner = mock_info("person", &[]);
    execute(deps.as_mut(), mock_env(), owner.clone(), approve_msg)
        .unwrap();

    let revoke_msg = ExecuteMsg::Revoke {
        spender: String::from("random"),
        token_id,
    };
    execute(deps.as_mut(), mock_env(), owner, revoke_msg)
        .unwrap();

    // Approvals are now removed / cleared
    let res: OwnerOfResponse = from_binary(
        &query(deps.as_ref(), mock_env(), query_msg)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        res,
        OwnerOfResponse {
            owner: String::from("person"),
            approvals: vec![],
        }
    );
}

#[test]
fn approving_all_revoking_all() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let token_uri1 = "https://www.merriam-webster.com/dictionary/grow1".to_string();

    let token_id2 = "grow2".to_string();
    let token_uri2 = "https://www.merriam-webster.com/dictionary/grow2".to_string();

    let mint_msg1 = ExecuteMsg::Mint(MintMsg {
        token_id: token_id1.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri1),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let minter = mock_info(MINTER, &[]);
    execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg1)
        .unwrap();

    let mint_msg2 = ExecuteMsg::Mint(MintMsg {
        token_id: token_id2.clone(),
        owner: String::from("demeter"),
        token_uri: Some(token_uri2),
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    execute(deps.as_mut(), mock_env(), minter, mint_msg2)
        .unwrap();

    // paginate the token_ids
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(1)).unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id1.clone()], tokens.tokens);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(token_id1.clone()), Some(3))
        .unwrap();
    assert_eq!(1, tokens.tokens.len());
    assert_eq!(vec![token_id2.clone()], tokens.tokens);

    // demeter gives random full (operator) power over her tokens
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("random"),
        expires: None,
    };
    let owner = mock_info("demeter", &[]);
    let res = execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "approve_all")
            .add_attribute("sender", "demeter")
            .add_attribute("operator", "random")
    );

    // random can now transfer
    let random = mock_info("random", &[]);
    let transfer_msg = ExecuteMsg::TransferNft {
        recipient: String::from("person"),
        token_id: token_id1,
    };
    execute(deps.as_mut(), mock_env(), random.clone(), transfer_msg)
        .unwrap();

    // random can now send
    let inner_msg = WasmMsg::Execute {
        contract_addr: "another_contract".into(),
        msg: to_binary("You now also have the growing power").unwrap(),
        funds: vec![],
    };
    let msg: CosmosMsg = CosmosMsg::Wasm(inner_msg);

    let send_msg = ExecuteMsg::SendNft {
        contract: String::from("another_contract"),
        token_id: token_id2,
        msg: to_binary(&msg).unwrap(),
    };
    execute(deps.as_mut(), mock_env(), random, send_msg)
        .unwrap();

    // Approve_all, revoke_all, and check for empty, to test revoke_all
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("operator"),
        expires: None,
    };
    // person is now the owner of the tokens
    let owner = mock_info("person", &[]);
    execute(deps.as_mut(), mock_env(), owner, approve_all_msg)
        .unwrap();

    let res = contract
        .all_approvals(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovedForAllResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    // second approval
    let buddy_expires = Expiration::AtHeight(1234567);
    let approve_all_msg = ExecuteMsg::ApproveAll {
        operator: String::from("buddy"),
        expires: Some(buddy_expires),
    };
    let owner = mock_info("person", &[]);
    execute(deps.as_mut(), mock_env(), owner.clone(), approve_all_msg)
        .unwrap();

    // and paginate queries
    let res = contract
        .all_approvals(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            None,
            Some(1),
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovedForAllResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );
    let res = contract
        .all_approvals(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            true,
            Some(String::from("buddy")),
            Some(2),
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovedForAllResponse {
            operators: vec![cw721::Approval {
                spender: String::from("operator"),
                expires: Expiration::Never {}
            }]
        }
    );

    let revoke_all_msg = ExecuteMsg::RevokeAll {
        operator: String::from("operator"),
    };
    execute(deps.as_mut(), mock_env(), owner, revoke_all_msg)
        .unwrap();

    // Approvals are removed / cleared without affecting others
    let res = contract
        .all_approvals(
            deps.as_ref(),
            mock_env(),
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(
        res,
        ApprovedForAllResponse {
            operators: vec![cw721::Approval {
                spender: String::from("buddy"),
                expires: buddy_expires,
            }]
        }
    );

    // ensure the filter works (nothing should be here
    let mut late_env = mock_env();
    late_env.block.height = 1234568; //expired
    let res = contract
        .all_approvals(
            deps.as_ref(),
            late_env,
            String::from("person"),
            false,
            None,
            None,
        )
        .unwrap();
    assert_eq!(0, res.operators.len());
}

#[test]
fn query_tokens_by_owner() {
    let mut deps = mock_dependencies(&[]);
    let contract = setup_contract(deps.as_mut());
    let minter = mock_info(MINTER, &[]);

    // Mint a couple tokens (from the same owner)
    let token_id1 = "grow1".to_string();
    let demeter = String::from("Demeter");
    let token_id2 = "grow2".to_string();
    let ceres = String::from("Ceres");
    let token_id3 = "sing".to_string();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id1.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id2.clone(),
        owner: ceres.clone(),
        token_uri: None,
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    execute(deps.as_mut(), mock_env(), minter.clone(), mint_msg)
        .unwrap();

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id3.clone(),
        owner: demeter.clone(),
        token_uri: None,
        extension: None,
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    execute(deps.as_mut(), mock_env(), minter, mint_msg)
        .unwrap();

    // get all tokens in order:
    let expected = vec![token_id1.clone(), token_id2.clone(), token_id3.clone()];
    let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    assert_eq!(&expected, &tokens.tokens);
    // paginate
    let tokens = contract.all_tokens(deps.as_ref(), None, Some(2)).unwrap();
    assert_eq!(&expected[..2], &tokens.tokens[..]);
    let tokens = contract
        .all_tokens(deps.as_ref(), Some(expected[1].clone()), None)
        .unwrap();
    assert_eq!(&expected[2..], &tokens.tokens[..]);

    // get by owner
    let by_ceres = vec![token_id2];
    let by_demeter = vec![token_id1, token_id3];
    // all tokens by owner
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, None)
        .unwrap();
    assert_eq!(&by_demeter, &tokens.tokens);
    let tokens = contract.tokens(deps.as_ref(), ceres, None, None).unwrap();
    assert_eq!(&by_ceres, &tokens.tokens);

    // paginate for demeter
    let tokens = contract
        .tokens(deps.as_ref(), demeter.clone(), None, Some(1))
        .unwrap();
    assert_eq!(&by_demeter[..1], &tokens.tokens[..]);
    let tokens = contract
        .tokens(deps.as_ref(), demeter, Some(by_demeter[0].clone()), Some(3))
        .unwrap();
    assert_eq!(&by_demeter[1..], &tokens.tokens[..]);
}





#[test]
fn storing_the_public_key_with_mint() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());

    let token_id = "someidwithpublickey".to_string();
    // let token_addr = Addr::unchecked("someidwithpublickey");
    let token_uri = "https://www.merriam-webster.com/dictionary/someidwithpublickey".to_string();

    let test_public_token_key = "1".to_string();
    let test_public_token_key_version = 1;

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("whoever"),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(100000u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..Metadata::default()
        }),
        token_key: Some(test_public_token_key.clone()),
        token_key_version: Some(test_public_token_key_version),
        is_tag: None,
        parent_tag_id: None,
    });

    let allowed = mock_info("whoever", &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg).unwrap();

    // it worked, let's query the public key (visible to anybody)
    let media_addr = Addr::unchecked(token_id.clone());
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPublicKey { media: media_addr.clone() }).unwrap();
    let value: PublicKeyResponse = from_binary(&res).unwrap();

    assert_eq!(test_public_token_key, value.token_key);
    assert_eq!(test_public_token_key_version, value.token_key_version);



}












#[test]
fn minting_the_private_tag() {
    let mut deps = mock_dependencies(&[]);
    setup_contract(deps.as_mut());


    // no tags at all for now
    let res = query(deps.as_ref(), mock_env(), QueryMsg::AllTags { start_after: None, limit: None}).unwrap();
    let value: TagsResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.tags.len());

    let tag_id = "testtagid";

    let mint_tag_msg = ExecuteMsg::MintTag(MintTagMsg {
        tag_id: Addr::unchecked(tag_id),
        is_private: true,
    });

    let allowed = mock_info("whoever", &[]);
    let _ = execute(deps.as_mut(), mock_env(), allowed.clone(), mint_tag_msg).unwrap();

    // there is 1 tag now
    let res = query(deps.as_ref(), mock_env(), QueryMsg::AllTags { start_after: None, limit: None}).unwrap();
    let value: TagsResponse = from_binary(&res).unwrap();
    assert_eq!(1, value.tags.len());
    assert_eq!(tag_id, value.tags[0]);

    // there is 1 tag for this owner now
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Tags { owner: "whoever".to_string(), start_after: None, limit: None}).unwrap();
    let value: TagsResponse = from_binary(&res).unwrap();
    assert_eq!(1, value.tags.len());
    assert_eq!(tag_id, value.tags[0]);

    // but still nothing for other addresses
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Tags { owner: "some_other_minter".to_string(), start_after: None, limit: None}).unwrap();
    let value: TagsResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.tags.len());



    // tokens list in this tag is empty for now
    let res = query(deps.as_ref(), mock_env(), QueryMsg::TagTokens { tag: Addr::unchecked(tag_id), start_after: None, limit: None}).unwrap();
    let value: TokensResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.tokens.len());


    // quick check that as tag is private, other users can not mint the token into it

    let token_id = "someidwithpublickey".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/someidwithpublickey".to_string();


    let disallowed = mock_info("somebody", &[]);

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("somebody"),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(100000u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            tag_id: Some(Addr::unchecked(tag_id)),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });

    let res = execute(deps.as_mut(), mock_env(), disallowed, mint_msg.clone());
    match res {
        Err(ContractError::Unauthorized {}) => {}
        _ => panic!("Must return unauthorized error"),
    }

    let mint_ok_msg = ExecuteMsg::Mint(MintMsg {
        token_id: token_id.clone(),
        owner: String::from("whoever"),
        token_uri: Some(token_uri.clone()),
        extension: Some(Metadata {
            watch_price: Some(Uint128::from(100000u128)), // 0.1 Luna
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            tag_id: Some(Addr::unchecked(tag_id)),
            ..Metadata::default()
        }),
        token_key: None,
        token_key_version: None,
        is_tag: None,
        parent_tag_id: None,
    });


    // but tag owner can!
    let _ = execute(deps.as_mut(), mock_env(), allowed, mint_ok_msg).unwrap();



    // tokens list in this tag has 1 item now
    let res = query(deps.as_ref(), mock_env(), QueryMsg::TagTokens { tag: Addr::unchecked(tag_id), start_after: None, limit: None}).unwrap();
    let value: TokensResponse = from_binary(&res).unwrap();
    assert_eq!(1, value.tokens.len());
    assert_eq!(token_id, value.tokens[0]);

    // nothing for others tags
    let res = query(deps.as_ref(), mock_env(), QueryMsg::TagTokens { tag: Addr::unchecked("someothertagid"), start_after: None, limit: None}).unwrap();
    let value: TokensResponse = from_binary(&res).unwrap();
    assert_eq!(0, value.tokens.len());

    // // list the token_ids
    // let tokens = contract.all_tokens(deps.as_ref(), None, None).unwrap();
    // assert_eq!(1, tokens.tokens.len());
    // assert_eq!(vec![token_id], tokens.tokens);

    // let test_public_token_key = "1".to_string();
    // let test_public_token_key_version = 1;

    // let mint_msg = ExecuteMsg::Mint(MintMsg {
    //     token_id: token_id.clone(),
    //     owner: String::from("whoever"),
    //     token_uri: Some(token_uri.clone()),
    //     extension: Some(Metadata {
    //         watch_price: Some(Uint128::from(100000u128)), // 0.1 Luna
    //         description: Some("Spaceship with Warp Drive".into()),
    //         name: Some("Starship USS Enterprise".to_string()),
    //         ..Metadata::default()
    //     }),
    //     token_key: Some(test_public_token_key.clone()),
    //     token_key_version: Some(test_public_token_key_version),
    //     is_tag: None,
    //     parent_tag_id: None,
    // });

    // let allowed = mock_info("whoever", &[]);
    // let _ = execute(deps.as_mut(), mock_env(), allowed, mint_msg).unwrap();

    // // it worked, let's query the public key (visible to anybody)
    // let media_addr = Addr::unchecked(token_id.clone());
    // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPublicKey { media: media_addr.clone() }).unwrap();
    // let value: PublicKeyResponse = from_binary(&res).unwrap();

    // assert_eq!(test_public_token_key, value.token_key);
    // assert_eq!(test_public_token_key_version, value.token_key_version);



}