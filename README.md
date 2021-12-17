# PepperWatch Smart Contract

CosmWasm smart contract for pay per view nft platform PepperWatch.

Extended cw721-base contract with custom methods for storing-purchasing keys for hidden NFT media and withdrawing earned crypto.

### Query Methods

```rust
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

    GetCount {},

    // Get Encoded Key For NFT with id of `media`
    GetKey { media: Addr, addr: Addr },

    // Get earnings balance for specific wallet
    GetBalance { addr: Addr },

    // Get minimum price to be set for per-view purchase. Default is 0 (no minimum)
    GetMinimumPrice { },

    // Get price to watch NFT with if of `media`
    GetPrice { media: Addr },

    // Get public key of `media` - to be decoded by validator and re-posted re-encoded for user purchased the view
    // We store the value encoded with RSA-OAEP, so only validators have the private keys for it
    GetPublicKey { media: Addr },
}
```

### Execute Methods
```rust
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

    /// Mint a new NFT, anybody can call it
    Mint(AsMintMsg<T>),

    /// Burn an NFT the sender has access to
    Burn { token_id: String },

    /// Our custom methods:
    ///
    /// Specify minimum per view price in uluna
    /// SetMinimumPrice - may be called by contract owner only. Price is in uluna (1/1000000) of Luna
    SetMinimumPrice { price: Uint128 },
    /// Per view price for specific nft:
    /// SetPrice - may be called by user minted specific NFT only. Price is in uluna (1/1000000) of Luna
    SetPrice { media: Addr, price: Uint128 },
    /// Purchase the key
    /// media - NFT token_id
    /// key - public key of x25519-xsalsa20-poly1305 user's keypair (check Metamask docs for detailed info on this)
    AskForKey { media: Addr, key: String },
    /// Deploy the key for user
    /// This called by validators
    /// media - NFT token id
    /// addr - the one purchased the view
    /// key - x25519-xsalsa20-poly1305 encoded key for the NFT
    FillKey { media: Addr, addr: Addr, key: String },

    /// Withdraw earnings. Sends all earned uluna (if any) to sender.address
    Withdraw {  },
}
```

### Developing

#### Prerequisites

```sh
rustc --version
cargo --version
rustup target list --installed
# if wasm32 is not listed above, run this
rustup target add wasm32-unknown-unknown
```

#### Compiling and running tests


```sh
# Run tests
cargo test

# Build
cargo wasm

# Optimize the build
cargo run-script optimize

# auto-generate json schema
cargo schema
```

#### Gas Estimation Script

Run (LocalTerra)[https://github.com/terra-money/LocalTerra], be sure you have test1 key set for `terrad`.

And run from this directory:
```sh
    node testfees.js

```

It will deploy currently compiled contract to localterra and execute common commands over it (mint, ask for key, fill the key) and display some gas info.

Working to optimize rust code based on this.


#### Deploying to blockchain

Actually, it's easier to do this from Terra Station's (contracts page)[https://station.terra.money/contracts]. Be sure you are connected to testnet or localterra.

- Upload the code
- Check code number uploaded on successful transaction
- Create contract using code number and InstantiateMsg:

```json
{
	"name": "Name", "symbol": "SMB", "minter": "yourterrawalletaddress"
}

```


