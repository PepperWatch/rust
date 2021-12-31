
// use cosmwasm_std::{Deps, Addr, StdResult, Uint256};
use cosmwasm_std::{Deps, Addr, StdResult, Order};
pub use cw721_base::{ContractError as Cw721ContractError,InstantiateMsg, MinterResponse};
pub use crate::msg::{Extension, Metadata, ExecuteMsg, MintMsg, QueryMsg, CountResponse, KeyResponse, BalanceResponse, PriceResponse, PublicKeyResponse, TagsResponse};
use crate::state::{STATE, MEDIA_KEY, BALANCE_HOLDER, MEDIA_PUBLIC_KEY, TAG};

use cw_storage_plus::Bound;

const DEFAULT_LIMIT: u32 = 10; // copied from cw721 -> query.rs
const MAX_LIMIT: u32 = 30;


pub fn query_tags(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TagsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);

    let tags: StdResult<Vec<Addr>> = TAG
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        // .map(|item| item.map(|(k, _)| k))
        .map(|x| x.map(|(_addr, item)| item.tag_id))
        .collect();

    Ok(TagsResponse { tags: tags? })
}


pub fn query_minimum_price(deps: Deps) -> StdResult<PriceResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(PriceResponse { uluna: state.minimum_watch_price.to_string() })
}

pub fn query_balance(deps: Deps, addr: Addr) -> StdResult<BalanceResponse> {
    let has_already = BALANCE_HOLDER
        .has(deps.storage, &addr);

    if has_already {
        let balance_holder = BALANCE_HOLDER
            .load(deps.storage, &addr);

        if balance_holder.is_ok() {
            let mut balance_holder = balance_holder.ok().unwrap();
            Ok(BalanceResponse { uluna: balance_holder.balance.uluna_as_string() })
        } else {
            Ok(BalanceResponse { uluna: "not ok".to_string() })
        }

    } else {
        Ok(BalanceResponse { uluna: "dont have".to_string() })
    }
}

pub fn query_key(deps: Deps, media: Addr, addr: Addr) -> StdResult<KeyResponse> {

    let has_already = MEDIA_KEY
        .has(deps.storage, (&addr, &media));


    if has_already {
        let media_key = MEDIA_KEY
            .load(deps.storage, (&addr, &media));

        if media_key.is_ok() {
            let m_key = media_key.ok().unwrap();
            Ok(KeyResponse { key: m_key.key, is_public: m_key.is_public })
        } else {
            Ok(KeyResponse { key: "".to_string(), is_public: false })
        }

    } else {
        Ok(KeyResponse { key: "".to_string(), is_public: false })
    }
}

pub fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count, count_filled: state.count_filled })
}

pub fn query_public_key(deps: Deps, media: Addr) -> StdResult<PublicKeyResponse> {
    let has_already = MEDIA_PUBLIC_KEY
        .has(deps.storage, &media);

    if has_already {
        let media_public_key = MEDIA_PUBLIC_KEY
            .load(deps.storage, &media);

        if media_public_key.is_ok() {
            let m_public_key = media_public_key.ok().unwrap();
            Ok(PublicKeyResponse { token_key: m_public_key.token_key, token_key_version: m_public_key.token_key_version })
        } else {
            Ok(PublicKeyResponse { token_key: "".to_string(), token_key_version: 0 })
        }
// Uint128::from(0u128)
    } else {
        Ok(PublicKeyResponse { token_key: "".to_string(), token_key_version: 0 })
    }
}