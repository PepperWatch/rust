use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use cosmwasm_std::{Addr, Binary, Uint128, Uint256};
use cosmwasm_std::{Addr, Binary, Uint128};
use cw721::{Expiration};


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,

    pub original_owner: Option<Addr>,
    pub watch_price: Option<Uint128>,
}

pub type Extension = Option<Metadata>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AsMintMsg<T> {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    pub token_key: Option<String>,
    pub token_key_version: Option<u32>,
    /// Any custom extension used by this contract
    pub extension: T,
}

pub type MintMsg = AsMintMsg<Extension>;

/// This is like Cw721ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AsExecuteMsg<T> {
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    /// Mint a new NFT, can only be called by the contract minter
    Mint(AsMintMsg<T>),

    /// Burn an NFT the sender has access to
    Burn { token_id: String },


    /// Our custom methods:
    ///
    SetPrice { media: Addr, price: Uint128 },
    AskForKey { media: Addr, key: String },
    FillKey { media: Addr, addr: Addr, key: String },

    Withdraw {  },
}

pub type ExecuteMsg = AsExecuteMsg<Extension>;

impl ExecuteMsg {
    pub fn into_cw721(self) -> cw721_base::msg::ExecuteMsg<Extension> {
        match self {
            ExecuteMsg::TransferNft { recipient, token_id } => cw721_base::msg::ExecuteMsg::TransferNft { recipient, token_id },
            ExecuteMsg::SendNft { contract, token_id, msg } => cw721_base::msg::ExecuteMsg::SendNft { contract, token_id, msg },
            ExecuteMsg::Approve { spender, token_id, expires } => cw721_base::msg::ExecuteMsg::Approve { spender, token_id, expires },
            ExecuteMsg::Revoke { spender, token_id } => cw721_base::msg::ExecuteMsg::Revoke { spender, token_id },
            ExecuteMsg::ApproveAll { operator, expires } => cw721_base::msg::ExecuteMsg::ApproveAll { operator, expires },
            ExecuteMsg::RevokeAll { operator } => cw721_base::msg::ExecuteMsg::RevokeAll { operator },
            // ExecuteMsg::Burn { token_id } => cw721_base::msg::ExecuteMsg::Burn { token_id },
            _ => panic!("unimplemented methods")
        }
    }
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum ExecuteMsg {
//     // Increment {},
//     // Reset { count: i32 },
//     AskForKey { media: Addr, key: String },
//     FillKey { media: Addr, addr: Addr, key: String },
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return the owner of the given token, error if token does not exist
    /// Return type: OwnerOfResponse
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    /// Return type: `ApprovedForAllResponse`
    ApprovedForAll {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract: `NftInfoResponse`
    NftInfo {
        token_id: String,
    },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients: `AllNftInfo`
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    /// Return type: TokensResponse.
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    // Return the minter
    Minter {},

    // GetCount returns the current count as a json-encoded number
    GetCount {},
    GetKey { media: Addr, addr: Addr },
    GetBalance { addr: Addr },
    GetPrice { media: Addr },
    GetPublicKey { media: Addr },
}

impl QueryMsg {
    pub fn into_cw721(self) -> cw721_base::msg::QueryMsg {
        match self {
            QueryMsg::OwnerOf { token_id, include_expired } => cw721_base::msg::QueryMsg::OwnerOf { token_id, include_expired },
            QueryMsg::ApprovedForAll { owner, include_expired, start_after, limit } => cw721_base::msg::QueryMsg::ApprovedForAll { owner, include_expired, start_after, limit },
            QueryMsg::NumTokens {  } => cw721_base::msg::QueryMsg::NumTokens {  },
            QueryMsg::ContractInfo {  } => cw721_base::msg::QueryMsg::ContractInfo {  },
            QueryMsg::NftInfo { token_id } => cw721_base::msg::QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo { token_id, include_expired } => cw721_base::msg::QueryMsg::AllNftInfo { token_id, include_expired },
            QueryMsg::Tokens { owner, start_after, limit } => cw721_base::msg::QueryMsg::Tokens { owner, start_after, limit },
            QueryMsg::AllTokens { start_after, limit } => cw721_base::msg::QueryMsg::AllTokens { start_after, limit },
            QueryMsg::Minter {  } => cw721_base::msg::QueryMsg::Minter {  },
            _ => panic!("unimplemented methods")
        }
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
    pub count_filled: i32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct KeyResponse {
    pub key: String,
    pub is_public: bool,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PublicKeyResponse {
    pub token_key: String,
    pub token_key_version: u32,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub uluna: String,
}


// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub uluna: String,
}




// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct WithdrawResponse {
    amount: Uint128,
}
