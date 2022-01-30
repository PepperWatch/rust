
mod tests;

pub mod state;
pub mod msg;
pub mod error;
pub mod queries;


mod local_cw721_base;
pub use local_cw721_base::state::{TokenInfo};

use cosmwasm_std::{Empty,DepsMut,Addr, Coin, Uint128, to_binary, Decimal,BankMsg,SubMsg,StdError};
pub use local_cw721_base::{ContractError as Cw721ContractError,InstantiateMsg, MinterResponse};
// pub use cw721_base::state::{TokenInfo};

pub use cw721::{ TokensResponse, };

// use cosmwasm_std::{Deps, Addr, StdResult};

// use cw20::{Balance};
use cw0::{NativeBalance};

pub use crate::error::ContractError;

pub use crate::msg::{Extension, Metadata, ExecuteMsg, MintMsg, MintTagMsg, QueryMsg, CountResponse, KeyResponse, BalanceResponse, TagsResponse, TagInfoResponse, PriceResponse, PublicKeyResponse, WithdrawResponse};

pub type PepperContract<'a> = local_cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type TokenInfoWithExtension = TokenInfo<Extension>;



trait PepperMethods {
    fn purchase_price(&self) -> Uint128;
    fn watch_price(&self, deps: &mut DepsMut, token_id: String, maybe_for_address: Option<Addr>) -> Uint128;
    fn nft_owner_addr(&self, deps: &mut DepsMut, token_id: String) -> Addr;
    fn nft_original_owner_addr(&self, deps: &mut DepsMut, token_id: String) -> Addr;
    fn count_nfts_in_colletion(&self, deps: &mut DepsMut, owner: Addr, collection_address: Addr) -> Result<usize, StdError>;

	// fn query_price(&self, deps: Deps, token_id: String) -> StdResult<PriceResponse>
}

impl PepperMethods for PepperContract<'_> {
    fn purchase_price(&self) -> Uint128 {
		return Uint128::from(1000000u128); // 1million uluna == 1 luna
    }
    fn count_nfts_in_colletion(&self, deps: &mut DepsMut, owner: Addr, collection_address: Addr) -> Result<usize, StdError> {
        let other_collection_nfts : TokensResponse = deps.querier.query_wasm_smart(
                collection_address,
                &QueryMsg::Tokens{
                    owner: owner.to_string(),
                    start_after: None,
                    limit: None,
                },
            ) ? ;

        Ok(other_collection_nfts.tokens.len())
    }
    fn watch_price(&self, deps: &mut DepsMut, token_id: String, maybe_for_address: Option<Addr>) -> Uint128 {
    	let token_info = self.tokens.load(deps.storage, &token_id.to_string());

    	if token_info.is_ok() {
    		let token_info = token_info.unwrap();
			let extension = token_info.extension.unwrap_or_default();
    		// minimum_amount = get_watch_price(&mut deps, token_info);
    		let price = extension.watch_price.unwrap();

		    if let Some(for_address) = maybe_for_address {
		    	if for_address == token_info.owner {
		    		// if it's the nft owner purchasing the key
		    		// it's for free
		    		return Uint128::from(0u128);
		    	}
		    }

    		return price;
    	} else {
    		return self.purchase_price();
    	}
    }
    fn nft_owner_addr(&self, deps: &mut DepsMut, token_id: String) -> Addr {
    	let token_info = self.tokens.load(deps.storage, &token_id.to_string());

    	if token_info.is_ok() {
    		let token_info = token_info.unwrap();
    		return Addr::unchecked(token_info.owner);
    	} else {
	        let minter = self.minter.load(deps.storage).ok().unwrap();
    		return minter;
    	}
    }
    fn nft_original_owner_addr(&self, deps: &mut DepsMut, token_id: String) -> Addr {
    	let token_info = self.tokens.load(deps.storage, &token_id.to_string());

    	if token_info.is_ok() {
    		let token_info = token_info.unwrap();
			let extension = token_info.extension.unwrap_or_default();
    		// minimum_amount = get_watch_price(&mut deps, token_info);
    		let original_owner = extension.original_owner.unwrap();
    		return original_owner;
    	} else {
	        let minter = self.minter.load(deps.storage).ok().unwrap();
    		return minter;
    	}
    }
	// fn query_price(&self, deps: Deps, token_id: String) -> StdResult<PriceResponse> {
 //    	let token_info = self.tokens.load(deps.storage, &token_id.to_string());
	// }
}



pub type Cw712ExecuteMessage = cw721_base::msg::ExecuteMsg<Extension>;

// use crate::state::{State, STATE, MEDIA_KEY, MediaKey};
use crate::state::{State, STATE, MEDIA_KEY, MediaKey, BALANCE_HOLDER, BalanceHolder, GenericBalance, MEDIA_PUBLIC_KEY, MediaPublicKey, tags, Tag };



use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
	    let state = State {
	        count: 0,
	        count_filled: 0,
	        minimum_watch_price: Uint128::from(0u128),
	        owner: info.sender.clone(),
	    };
	    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
	    STATE.save(deps.storage, &state)?;

        PepperContract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
	    match msg {
	        ExecuteMsg::SetMinimumPrice { price } => try_set_minimum_price(deps, info, price),
	        ExecuteMsg::SetPrice { media, price } => try_set_price(deps, info, media, price),
	        ExecuteMsg::AskForKey { media, key } => try_ask_for_key(deps, info, media, key),
	        ExecuteMsg::FillKey { media, addr, key } => try_fill_key(deps, info, media, addr, key),
	        ExecuteMsg::Withdraw {  } => try_withdraw(deps, env, info),
	        ExecuteMsg::Mint(msg) => try_mint(deps, info, msg),
	        ExecuteMsg::MintTag(msg) => try_mint_tag(deps, info, msg),
            ExecuteMsg::BurnTag { tag_id } => try_burn_tag(deps, info, tag_id),
	        ExecuteMsg::Burn { token_id } => try_burn(deps, env, info, token_id),
            // ExecuteMsg::CheckCollection { collection } => try_check_collection(deps, env, info, collection),
	        _ => default_execute_to_extended(deps, env, info, msg)
	    }
    }

    fn default_execute_to_extended(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
    	let res = PepperContract::default().execute(deps, env, info, msg.into_cw721());

	    match res {
	        Err(Cw721ContractError::Unauthorized {}) => Err(ContractError::Unauthorized {}),
	        Err(Cw721ContractError::Claimed {}) => Err(ContractError::Claimed {}),
	        Err(Cw721ContractError::Expired {}) => Err(ContractError::Expired {}),
			Ok(val) => Ok(val),
	        Err(_err) => Err(ContractError::Unhandled {}),
	    }
    }

    // pub fn try_check_collection(mut deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     collection: Addr
    // ) -> Result<Response, ContractError> {
    //     let original_contract = PepperContract::default();

    //     let count = original_contract.count_nfts_in_colletion(&mut deps, info.sender.clone(), collection.clone());

    //     let mut resp_count = 0;

    //     if count.is_ok() {
    //         resp_count = count.unwrap();
    //     }

    //     Ok(Response::new()
    //         .add_attribute("action", "try_check_collection")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("count", resp_count.to_string()))
    // }

    pub fn try_burn(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response, ContractError> {
    	let original_contract = PepperContract::default();
    	let token_info = original_contract.tokens.load(deps.storage, &token_id)?;

		if token_info.owner != info.sender {
            return Err(ContractError::Unauthorized {});
		}

        original_contract.tokens.remove(deps.storage, &token_id)?;

        // decrement tokens original_contract.decrement_tokens(deps.storage)?;
        let val = original_contract.token_count(deps.storage)? - 1;
        original_contract.token_count.save(deps.storage, &val)?;

        Ok(Response::new()
            .add_attribute("action", "burn")
            .add_attribute("sender", info.sender)
            .add_attribute("token_id", token_id))
    }

    pub fn try_withdraw(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
    	let addr = info.sender.clone();
	    let has_already = BALANCE_HOLDER
	        .has(deps.storage, &addr);

	    if has_already {
	        let balance_holder = BALANCE_HOLDER
	            .load(deps.storage, &addr);

	        if balance_holder.is_ok() {
	        	let amount = balance_holder.unwrap().balance.uluna();

			    // let mut messages: Vec<CosmosMsg> = vec![];
			    let withdraw_coins: Vec<Coin> = vec![Coin {
			        denom: "uluna".to_string(),
			        amount: amount,
			    }];


		        BALANCE_HOLDER.update(deps.storage, &addr, |old| -> StdResult<_> {
		            let mut balance_holder = old.unwrap();

		            balance_holder.balance.uluna_to_null();

		            Ok(balance_holder)
		        }).ok();

				// let bank_send = CosmosMsg::Bank(BankMsg::Send {
				// 	to_address: info.sender.clone().into(),
				// 	amount: withdraw_coins,
				// });



                // Send principal back to sender
                return Ok(Response::new()
                    .add_submessage(SubMsg::new(BankMsg::Send {
                        to_address: info.sender.clone().into(),
                        amount: withdraw_coins,
                    }))
                    .add_attribute("action", "withdraw")
                    .add_attribute(
                        "info",
                        format!("withdrew {} LUNA", amount.to_string()),
                    ));
	        }
	    }

	    return Err(ContractError::Unauthorized {});
    }

    pub fn try_mint_tag(
        deps: DepsMut,
        info: MessageInfo,
        msg: MintTagMsg,
    ) -> Result<Response, ContractError> {

        // create the tag
        let tag = Tag {
            owner: info.sender.clone(),
            tag_id: msg.tag_id.clone(),
            count: 0,
            is_private: msg.is_private,
            main_token_id: None,
            for_owners_of: msg.for_owners_of,
            extra: msg.extra,

        };

        tags().update(deps.storage, &msg.tag_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(tag),
            })?;

        Ok(Response::new()
            .add_attribute("action", "mint_tag")
            .add_attribute("minter", info.sender)
            .add_attribute("tag_id", msg.tag_id))
    }


    pub fn try_burn_tag(
        deps: DepsMut,
        info: MessageInfo,
        tag_id: Addr,
    ) -> Result<Response, ContractError> {

        let tags = tags();
        let tag = tags.load(deps.storage, &tag_id)?;
        if tag.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }

        tags.remove(deps.storage, &tag_id)?;

        Ok(Response::new()
            .add_attribute("action", "burn_tag")
            .add_attribute("sender", info.sender)
            .add_attribute("tag_id", tag_id))
    }

    pub fn try_mint(
        deps: DepsMut,
        info: MessageInfo,
        msg: MintMsg,
    ) -> Result<Response, ContractError> {
    	let original_contract = PepperContract::default();

        // let minter = original_contract.minter.load(deps.storage)?;

        // letting anybody mint (@todo: ????)

        // if info.sender != minter {
        //     return Err(ContractError::Unauthorized {});
        // }

        let mut extension = msg.extension.unwrap_or_default();
        extension.original_owner = Some(info.sender.clone());

        // let price = extension.watch_price.unwrap_or_default();
        if extension.watch_price.is_some() {
        	let state = STATE.load(deps.storage)?;
        	let price_to_set = extension.watch_price.unwrap_or_default();

		    if price_to_set < state.minimum_watch_price {
	            return Err(ContractError::Invalid {});
		    }
        	extension.watch_price = Some(price_to_set); // take the price from message
        } else {
        	extension.watch_price = Some(original_contract.purchase_price()); // default one
        }

        let tag_id;
        if extension.tag_id.is_some() {
        	// minting the nft into some tag as parent

        	// 1st - load the tag information
        	//
        	tag_id = extension.tag_id.clone().unwrap();
	    	let tag_info = tags().load(deps.storage, &tag_id)?;

	    	// 2nd - check tag access settings
	    	if tag_info.is_private {
	    		// only tag owner can mint into this tag
	    		if tag_info.owner != msg.owner {
			        return Err(ContractError::Unauthorized {});
	    		}
	    	}

	    	// @todo: additional logic

	    	// 3rd - update tag items count
	        tags().update(deps.storage, &tag_id, |old| -> StdResult<_> {
	            let mut tag = old.unwrap();
	            tag.count = tag.count + 1;
	            Ok(tag)
	        })?;
        } else {
            tag_id = deps.api.addr_validate(&msg.owner)?;
        }

        let option_extension = Some(extension);

        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&msg.owner)?,
            tag: tag_id,
            approvals: vec![],
            token_uri: msg.token_uri,
            extension: option_extension,
        };

        original_contract.tokens
            .update(deps.storage, &msg.token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        original_contract.increment_tokens(deps.storage)?;

        // store media_public_key if passed
        if msg.token_key.is_some() {
	        if msg.token_key_version.is_some() {
		        let media_public_key = MediaPublicKey {
		            token_key: msg.token_key.unwrap(),
		            token_key_version: msg.token_key_version.unwrap(),
		        };
		        let token_id_as_addr = Addr::unchecked(&msg.token_id);
		        MEDIA_PUBLIC_KEY.save(deps.storage, &token_id_as_addr, &media_public_key)?;
	        }
        }


        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", info.sender)
            .add_attribute("token_id", msg.token_id))
    }


    pub fn try_set_minimum_price(deps: DepsMut, info: MessageInfo, price: Uint128) -> Result<Response, ContractError> {
        let contract_creator = PepperContract::default().minter.load(deps.storage).ok().unwrap();

        if info.sender != contract_creator {
	        return Err(ContractError::Unauthorized {});
        } else {
    		if price <= Uint128::from(0u128) {
    			// need to handle minimum price
	            return Err(ContractError::Invalid {});
    		}

            STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
                state.minimum_watch_price = price;
                Ok(state)
            })?;

		    Ok(Response::new().add_attribute("method", "try_set_minimum_price"))
        }
    }

    pub fn try_set_price(deps: DepsMut, info: MessageInfo, media: Addr, price: Uint128) -> Result<Response, ContractError> {
	    let state = STATE.load(deps.storage)?;

	    if price < state.minimum_watch_price {
            return Err(ContractError::Invalid {});
	    }

    	let contract = PepperContract::default();

    	let token_info = contract.tokens.load(deps.storage, &media.to_string());

    	if token_info.is_ok() {
    		let token_info = token_info.unwrap();

    		if token_info.owner != info.sender {
	            return Err(ContractError::Unauthorized {});
    		}

    		if price <= Uint128::from(0u128) {
    			// need to handle minimum price
	            return Err(ContractError::Invalid {});
    		}

	        contract.tokens.update(deps.storage, &media.to_string(), |old| -> StdResult<_> {
	            let mut token_info = old.unwrap();
		        let mut extension = token_info.extension.unwrap_or_default();
		        extension.watch_price = Some(price);
		        token_info.extension = Some(extension);

	            Ok(token_info)
	        })?;


		    Ok(Response::new().add_attribute("method", "try_set_price"))
    	} else {
            return Err(ContractError::Unauthorized {});
    	}
    }

	pub fn try_fill_key(deps: DepsMut, info: MessageInfo, media: Addr, addr: Addr, key: String) -> Result<Response, ContractError> {
	    let state = STATE.load(deps.storage)?;

	    if info.sender != state.owner {
	        return Err(ContractError::Unauthorized {});
	    }

	    // let has_already = MEDIA_KEY
	    //     .has(deps.storage, (&addr, &media));

	    // if has_already {
	        let mut updated = false;

	        MEDIA_KEY.update(deps.storage, (&addr, &media), |old| -> StdResult<_> {
	            let mut m_key = old.unwrap();

	            if m_key.is_public {
	                m_key.key = key;
	                m_key.is_public = false;

	                updated = true;
	            }

	            Ok(m_key)
	        })?;

	        // if updated {
	        //     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
	        //         state.count_filled += 1;
	        //         Ok(state)
	        //     })?;
	        // }

	    // }

	    Ok(Response::new().add_attribute("method", "try_fill_key"))
	}

	fn store_coins(deps: &mut DepsMut, addr: Addr, coin: Coin) {
	    let has_already = BALANCE_HOLDER
	        .has(deps.storage, &addr);

	    if !has_already {
	    	let addr_clone = addr.clone();
	    	let balance_holder = BalanceHolder {
	    		owner: addr_clone,
	    		balance: GenericBalance {
	                native: vec![coin],
		            cw20: vec![],
	    		},
	    	};

	        BALANCE_HOLDER.save(deps.storage, &addr, &balance_holder).ok();
	    } else {

	        BALANCE_HOLDER.update(deps.storage, &addr, |old| -> StdResult<_> {
	            let mut balance_holder = old.unwrap();

	            balance_holder.balance.add_coin(coin);

	            Ok(balance_holder)
	        }).ok();

	    }
	}

	// fn get_watch_price(deps: &mut DepsMut, token_info: TokenInfoWithExtension) -> Uint128 {
	// 	// let unwraped = token_info.unwrap();
	// 	let extension = token_info.extension.unwrap_or_default();

	// 	let price = extension.watch_price.unwrap();

	// 	return price
	// }

	pub fn try_ask_for_key(mut deps: DepsMut, info: MessageInfo, media: Addr, key: String) -> Result<Response, ContractError> {
	    let has_already = MEDIA_KEY
	        .has(deps.storage, (&info.sender, &media));

	    if !has_already {
	    	let contract = PepperContract::default();

	    	let minimum_amount = contract.watch_price(&mut deps, media.to_string(), Some(info.sender.clone()) );
            let state = STATE.load(deps.storage)?;

            let expected_amount = minimum_amount + state.minimum_watch_price;

	    	if !expected_amount.is_zero() {

                // check if it's allowed for current sender
                let token_info = contract.tokens.load(deps.storage, &media.to_string());

                if token_info.is_ok() {
                    let token_info = token_info.unwrap();
                    let extension = token_info.extension.unwrap();

                    // check if tag has settings that limit viewing of media for specific collection nft's holders
                    if extension.tag_id.is_some() {
                        let tag_id = extension.tag_id.unwrap();
                        let tags = tags();
                        let tag = tags.load(deps.storage, &tag_id)?;
                        if tag.for_owners_of.is_some() {
                            let count_nfts = contract.count_nfts_in_colletion(&mut deps, info.sender.clone(), Addr::unchecked(tag.for_owners_of.unwrap()));

                            if count_nfts.is_ok() {
                                if count_nfts.unwrap() < 1 {
                                    return Err(ContractError::Unauthorized {});
                                }
                            } else {
                                return Err(ContractError::Unauthorized {});
                            }
                        }
                    }
                }

		    	let balance = NativeBalance(info.funds);

		        let expected_coin = Coin { denom: "uluna".to_string(), amount: expected_amount };

		        if !balance.has(&expected_coin) {
		            return Err(ContractError::NotEnoughFunds {});
		        }

                // Send state.minimum_watch_price ... 10% to contract creator:

                let contract_creator = contract.minter.load(deps.storage).ok().unwrap();
                let percent = Decimal::percent(10u64);
                let amount = minimum_amount * percent + state.minimum_watch_price; // amount is same 10%, but when optimized, no extra gas, so lets keep it for reading
                let coint_to_store = Coin { denom: "uluna".to_string(), amount: amount };
                store_coins(&mut deps, contract_creator, coint_to_store);


		        // Send 80% to NFT's owner:

		        let percent = Decimal::percent(80u64);
			    let coints_to = contract.nft_owner_addr(&mut deps, media.to_string());
		        let amount = minimum_amount * percent;
		        let coint_to_store = Coin { denom: "uluna".to_string(), amount: amount };
		        store_coins(&mut deps, coints_to, coint_to_store);

		        // Send 10% to original NFT's owner (who minted it):

		        let percent = Decimal::percent(10u64);
			    let coints_to = contract.nft_original_owner_addr(&mut deps, media.to_string());
		        let amount = minimum_amount * percent;
		        let coint_to_store = Coin { denom: "uluna".to_string(), amount: amount };
		        store_coins(&mut deps, coints_to, coint_to_store);

	    	}

	        let owner = info.sender.clone();
	        let media_key = MediaKey {
	            owner: owner,
	            key: key,
	            is_public: true,
	        };

	        // save new MediaKey record
	        MEDIA_KEY.save(deps.storage, (&info.sender, &media), &media_key)?;

	        // Increment count in state
	        // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
	        //     state.count += 1;
	        //     Ok(state)
	        // })?;
	    }

	    Ok(Response::new().add_attribute("method", "try_ask_for_key"))
	}


	pub use crate::queries::{query_count, query_key, query_balance, query_public_key, query_minimum_price, query_all_tags, query_tags, query_tag_info, DEFAULT_LIMIT, MAX_LIMIT};


	pub fn query_price(deps: Deps, media: Addr) -> StdResult<PriceResponse> {
    	let contract = PepperContract::default();

    	let token_info = contract.tokens.load(deps.storage, &media.to_string());

    	if token_info.is_ok() {
    		let token_info = token_info.unwrap();
			let extension = token_info.extension.unwrap_or_default();
    		// minimum_amount = get_watch_price(&mut deps, token_info);
    		let price = extension.watch_price.unwrap();

    		Ok(PriceResponse { uluna: price.to_string() })
    	} else {
    		Ok(PriceResponse { uluna: contract.purchase_price().to_string() })
    	}
	}

    use cosmwasm_std::{Order, StdError, };
    use cw_storage_plus::Bound;
    use crate::local_cw721_base::query::TokensResponse;

    pub fn query_tag_tokens(deps: Deps, tag_id: Addr, start_after: Option<String>, limit: Option<u32>,) -> StdResult<TokensResponse> {
        let contract = PepperContract::default();

        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);

        let pks: Vec<_> = contract
            .tokens
            .idx
            .tag
            .prefix(tag_id)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect();

        let res: Result<Vec<_>, _> = pks.iter().map(|v| String::from_utf8(v.to_vec())).collect();
        let tokens = res.map_err(StdError::invalid_utf8)?;
        Ok(TokensResponse { tokens })
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    	// let contract = PepperContract::default();

	    match msg {
            QueryMsg::TagInfo { tag_id, } => to_binary(&query_tag_info(deps, tag_id)?),
            QueryMsg::TagTokens { tag_id, start_after, limit, } => to_binary(&query_tag_tokens(deps, tag_id, start_after, limit)?),
            QueryMsg::Tags { owner, start_after, limit, } => to_binary(&query_tags(deps, owner, start_after, limit)?),
	        QueryMsg::AllTags { start_after, limit, } => to_binary(&query_all_tags(deps, start_after, limit)?),
	        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
	        QueryMsg::GetKey { media, addr } => to_binary(&query_key(deps, media, addr)?),
	        QueryMsg::GetBalance { addr } => to_binary(&query_balance(deps, addr)?),
	        QueryMsg::GetPrice { media } => to_binary(&query_price(deps, media)?),
	        QueryMsg::GetMinimumPrice { } => to_binary(&query_minimum_price(deps)?),
	        QueryMsg::GetPublicKey { media } => to_binary(&query_public_key(deps, media)?),
	        // QueryMsg::GetKey {  } => try_ask_for_key(deps, info, media, key),
	        _ => PepperContract::default().query(deps, env, msg.into_cw721())
	    }
    }

}

// mut deps: DepsMut,