use pyra_accounts::*;
use pyra_tokens::SUPPORTED_TOKENS;
use solana_pubkey::{pubkey, Pubkey};

const OWNER: Pubkey = pubkey!("5DXPwzc9VurVGVZfpet9bbaL7XQvFSVz5AKQo7z11r4");

#[test]
fn test_get_vault() {
    let vault = get_vault(OWNER);
    assert_eq!(
        vault,
        pubkey!("75eLjyjszgrpqqL4udoyyRS57J1m6zKqSQ5EUm4Dgrjf")
    );
}

#[test]
fn test_get_deposit_address() {
    let deposit_address = get_deposit_address(OWNER);
    assert_eq!(
        deposit_address,
        pubkey!("HkpWHP7wuF6wYKKPEgNk35mL5hoG4LStfcFRnU4ur2oz")
    );
}

#[test]
fn test_get_send_hold() {
    let send_hold = get_spend_hold();
    assert_eq!(
        send_hold,
        pubkey!("CaMDMNNVqfqEJzALZ5TtMH19fTfhSnnXSW9EgCyJUGP")
    );
}

#[test]
fn test_get_swap_ledger() {
    // USDC
    let swap_ledger = get_swap_ledger(SUPPORTED_TOKENS[0].mint);
    assert_eq!(
        swap_ledger,
        pubkey!("5TkaghhdPNyNjJ6ucyFik25dWPgNx4SNzMZ9MShDJ7qu")
    );

    // wSOL
    let swap_ledger = get_swap_ledger(SUPPORTED_TOKENS[1].mint);
    assert_eq!(
        swap_ledger,
        pubkey!("7EGL54SwicuLda36Pjg7ih5pBCj6gG7MAHDgJyizAvH3")
    );
}

#[test]
fn test_get_drift_user() {
    let drift_user = get_drift_user(OWNER, 0);
    assert_eq!(
        drift_user,
        pubkey!("4fmAhVBhpQdbSc9SLhHvxxYRoxfWw9s2fUZq44vyfFTg")
    );
}

#[test]
fn test_get_drift_user_stats() {
    let drift_user_stats = get_drift_user_stats(OWNER);
    assert_eq!(
        drift_user_stats,
        pubkey!("5ZLpBrU2z3QL4CgZxmyYg8Zv8skJ82gP2v27fprMdcJs")
    );
}

#[test]
fn test_get_drift_state() {
    let drift_state = get_drift_state();
    assert_eq!(
        drift_state,
        pubkey!("5zpq7DvB6UdFFvpmBPspGPNfUGoBRRCE2HHg5u3gxcsN")
    );
}

#[test]
fn test_get_drift_signer() {
    let drift_signer = get_drift_signer();
    assert_eq!(
        drift_signer,
        pubkey!("JCNCMFXo5M5qwUPg2Utu1u6YWp3MbygxqBsBeXXJfrw")
    );
}

#[test]
fn test_get_drift_spot_market_vault() {
    // USDC
    let vault = get_drift_spot_market_vault(SUPPORTED_TOKENS[0].drift_market_index);
    assert_eq!(
        vault,
        pubkey!("GXWqPpjQpdz7KZw9p7f5PX2eGxHAhvpNXiviFkAB8zXg")
    );

    // wSOL
    let vault = get_drift_spot_market_vault(SUPPORTED_TOKENS[1].drift_market_index);
    assert_eq!(
        vault,
        pubkey!("DfYCNezifxAEsQbAJ1b3j6PX3JVBe8fu11KBhxsbw5d2")
    );
}

#[test]
fn test_get_drift_spot_market() {
    // USDC
    let market = get_drift_spot_market(SUPPORTED_TOKENS[0].drift_market_index);
    assert_eq!(
        market,
        pubkey!("6gMq3mRCKf8aP3ttTyYhuijVZ2LGi14oDsBbkgubfLB3")
    );

    // wSOL
    let market = get_drift_spot_market(SUPPORTED_TOKENS[1].drift_market_index);
    assert_eq!(
        market,
        pubkey!("3x85u7SWkmmr7YQGYhtjARgxwegTLJgkSLRprfXod6rh")
    );
}
