use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Uint128};
// use cosmwasm_std::{Addr, Coin, Uint128, Uint256};
use cw_storage_plus::{Item, Map, MultiIndex, Index, IndexedMap, IndexList};
use cw20::{Balance, Cw20CoinVerified};

// Key to decode specific media for specific user
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaKey {
    pub owner: Addr,			// owner of the key (watcher)
    pub key: String,	// key data itself
    pub is_public: bool,			// 1st step - true (watcher publicKey), 2nd step - false (encoded by publicKey media password)
}

// original media key encoded by validator ( so only validator can decode it to original )
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MediaPublicKey {
    pub token_key: String,
    pub token_key_version: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub count_filled: i32,
    pub owner: Addr,
    pub minimum_watch_price: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const MEDIA_PUBLIC_KEY: Map<&Addr, MediaPublicKey> = Map::new("media_public_key");
pub const MEDIA_KEY: Map<(&Addr, &Addr), MediaKey> = Map::new("media_key");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tag {
    pub count: i32,
    pub owner: Addr,
    pub main_token_id: Option<String>,
    pub tag_id: Addr,
    pub is_private: bool,
}

pub struct TagIndexes<'a> {
  pub owner: MultiIndex<'a, (Addr, Vec<u8>), Tag>,
}

impl<'a> IndexList<Tag> for TagIndexes<'a> {
  fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Tag>> + '_> {
    let v: Vec<&dyn Index<Tag>> = vec![&self.owner];
    Box::new(v.into_iter())
  }
}

pub fn tags<'a>() -> IndexedMap<'a, &'a Addr, Tag, TagIndexes<'a>> {
  let indexes = TagIndexes {
    owner: MultiIndex::new(
      |d: &Tag, k: Vec<u8>| (d.owner.clone(), k),
      "tags",
      "tags__owner",
    ),
  };
  IndexedMap::new("tags", indexes)
}

// pub const TAG: Map<&Addr, Tag> = Map::new("tag");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct GenericBalance {
    pub native: Vec<Coin>,
    pub cw20: Vec<Cw20CoinVerified>,
}

// NativeBalance - https://github.com/CosmWasm/cw-plus/blob/bf09c4d88673ac106c8e15e9c7c263b3c478843d/packages/cw0/src/balance.rs
// Balance - https://github.com/CosmWasm/cw-plus/blob/bf09c4d88673ac106c8e15e9c7c263b3c478843d/packages/cw20/src/balance.rs

impl GenericBalance {
    pub fn uluna_to_null(&mut self) {
        let index = self.native.iter().enumerate().find_map(|(i, exist)| {
            if exist.denom == "uluna"{
                Some(i)
            } else {
                None
            }
        });
        match index {
            Some(idx) => self.native[idx].amount = Uint128::from(0u128),
            None => self.native.push(Coin { denom: "uluna".to_string(), amount: Uint128::from(0u128) }),
        }
    }

    pub fn uluna(&mut self) -> Uint128 {
        let index = self.native.iter().enumerate().find_map(|(i, exist)| {
            if exist.denom == "uluna" {
                Some(i)
            } else {
                None
            }
        });

        let amount = match index {
            Some(idx) => self.native[idx].amount,
            None => Uint128::from(0u128),
        };

        return amount;
    }

	pub fn uluna_as_string(&mut self) -> String {
        let index = self.native.iter().enumerate().find_map(|(i, exist)| {
            if exist.denom == "uluna" {
                Some(i)
            } else {
                None
            }
        });

        let amount = match index {
            Some(idx) => self.native[idx].amount,
            None => Uint128::from(0u128),
        };

		return amount.to_string();
	}

	pub fn add_coin(&mut self, add: Coin) {
        let index = self.native.iter().enumerate().find_map(|(i, exist)| {
            if exist.denom == add.denom {
                Some(i)
            } else {
                None
            }
        });
        match index {
            Some(idx) => self.native[idx].amount += add.amount,
            None => self.native.push(add),
        }
	}

    pub fn add_tokens(&mut self, add: Balance) {
        match add {
            Balance::Native(balance) => {
                for token in balance.0 {
                    let index = self.native.iter().enumerate().find_map(|(i, exist)| {
                        if exist.denom == token.denom {
                            Some(i)
                        } else {
                            None
                        }
                    });
                    match index {
                        Some(idx) => self.native[idx].amount += token.amount,
                        None => self.native.push(token),
                    }
                }
            }
            Balance::Cw20(token) => {
                let index = self.cw20.iter().enumerate().find_map(|(i, exist)| {
                    if exist.address == token.address {
                        Some(i)
                    } else {
                        None
                    }
                });
                match index {
                    Some(idx) => self.cw20[idx].amount += token.amount,
                    None => self.cw20.push(token),
                }
            }
        };
    }
}

/// we keep balances in separate object. So they are kept if/when nft sold to another owner
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BalanceHolder {
    pub owner: Addr,
    pub balance: GenericBalance,
}

pub const BALANCE_HOLDER: Map<&Addr, BalanceHolder> = Map::new("balance_holder");