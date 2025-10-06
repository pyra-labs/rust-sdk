//! Token definitions for the Pyra Solana smart contract ecosystem.
//!
//! ```rust
//! use pyra_tokens::{Token, SUPPORTED_TOKENS};
//!
//! if let Some(token) = Token::find_by_market_index(0) {
//!     println!("USDC mint: {}", token.mint);
//! }
//! ```

use solana_program::{pubkey, pubkey::Pubkey};

mod helpers;

pub type PythFeedId = [u8; 32];
const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub name: &'static str,
    pub drift_market_index: u16,
    pub mint: Pubkey,
    pub token_program: Pubkey,
    pub decimals: u8,
    pub pyth_price_feed: PythFeedId,
    pub is_usd_stablecoin: bool,
}

pub const SUPPORTED_TOKENS: [Token; 13] = [
    Token {
        name: "wSOL",
        drift_market_index: 1,
        mint: pubkey!("So11111111111111111111111111111111111111112"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 9,
        pyth_price_feed: feed_id!(
            "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "USDC",
        drift_market_index: 0,
        mint: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a"
        ),
        is_usd_stablecoin: true,
    },
    Token {
        name: "USDT",
        drift_market_index: 5,
        mint: pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0x2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b"
        ),
        is_usd_stablecoin: true,
    },
    Token {
        name: "PYUSD",
        drift_market_index: 22,
        mint: pubkey!("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo"),
        token_program: TOKEN_2022_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0xc1da1b73d7f01e7ddd54b3766cf7fcd644395ad14f70aa706ec5384c59e76692"
        ),
        is_usd_stablecoin: true,
    },
    Token {
        name: "USDS",
        drift_market_index: 28,
        mint: pubkey!("USDSwr9ApdHk5bvJKMjzff41FfuX8bSxdKcR81vTwcA"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0x77f0971af11cc8bac224917275c1bf55f2319ed5c654a1ca955c82fa2d297ea1"
        ),
        is_usd_stablecoin: true,
    },
    Token {
        name: "wBTC",
        drift_market_index: 3,
        mint: pubkey!("3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 8,
        pyth_price_feed: feed_id!(
            "0xc9d8b075a5c69303365ae23633d4e085199bf5c520a3b90fed1322a0342ffc33"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "JitoSOL",
        drift_market_index: 6,
        mint: pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 9,
        pyth_price_feed: feed_id!(
            "0x67be9f519b95cf24338801051f9a808eff0a578ccb388db73b7f6fe1de019ffb"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "JLP",
        drift_market_index: 19,
        mint: pubkey!("27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0xc811abc82b4bad1f9bd711a2773ccaa935b03ecef974236942cec5e0eb845a3a"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "bSOL",
        drift_market_index: 8,
        mint: pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 9,
        pyth_price_feed: feed_id!(
            "0x89875379e70f8fbadc17aef315adf3a8d5d160b811435537e03c97e8aac97d9c"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "BONK",
        drift_market_index: 32,
        mint: pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 5,
        pyth_price_feed: feed_id!(
            "0x72b021217ca3fe68922a19aaf990109cb9d84e9ad004b4d2025ad6f529314419"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "JUP",
        drift_market_index: 11,
        mint: pubkey!("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 6,
        pyth_price_feed: feed_id!(
            "0x0a0408d619e9380abad35060f9192039ed5042fa6f82301d0e48bb52be830996"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "zBTC",
        drift_market_index: 45,
        mint: pubkey!("zBTCug3er3tLyffELcvDNrKkCymbPWysGcWihESYfLg"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 8,
        pyth_price_feed: feed_id!(
            "0x3d824c7f7c26ed1c85421ecec8c754e6b52d66a4e45de20a9c9ea91de8b396f9"
        ),
        is_usd_stablecoin: false,
    },
    Token {
        name: "wETH",
        drift_market_index: 4,
        mint: pubkey!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs"),
        token_program: TOKEN_PROGRAM_ID,
        decimals: 8,
        pyth_price_feed: feed_id!(
            "0x9d4294bbcd1174d6f2003ec365831e64cc31d9f6f15a2b85399db8d5000960f6"
        ),
        is_usd_stablecoin: false,
    },
];

impl Token {
    pub fn find_by_drift_market_index(drift_market_index: u16) -> Option<&'static Token> {
        SUPPORTED_TOKENS
            .iter()
            .find(|token| token.drift_market_index == drift_market_index)
    }

    pub fn find_by_mint(mint: &Pubkey) -> Option<&'static Token> {
        SUPPORTED_TOKENS.iter().find(|token| token.mint == *mint)
    }
}
