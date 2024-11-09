//! Tests Zainod release binary against the `zcash_local_net` client RPC test fixtures.
//!
//! Pre-requisites:
//! - Run `cargo build --release` to update to latest release binary
//! - Run `generate_zcashd_chain_cache` to generate the chain cache for testing.

use std::path::PathBuf;

use once_cell::sync::Lazy;

static ZCASHD_BIN: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let mut workspace_root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    workspace_root_path.pop();
    Some(workspace_root_path.join("test_binaries/bins/zcashd"))
});
static ZCASH_CLI_BIN: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let mut workspace_root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    workspace_root_path.pop();
    Some(workspace_root_path.join("test_binaries/bins/zcash-cli"))
});
static LIGHTWALLETD_BIN: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let mut workspace_root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    workspace_root_path.pop();
    Some(workspace_root_path.join("test_binaries/bins/lightwalletd"))
});
static ZAINOD_BIN: Lazy<Option<PathBuf>> = Lazy::new(|| {
    let mut workspace_root_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    workspace_root_path.pop();
    Some(workspace_root_path.join("target/release/zainod"))
});

#[ignore = "not a test. generates chain cache for client_rpc tests."]
#[tokio::test]
async fn generate_zcashd_chain_cache() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::generate_zcashd_chain_cache(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_lightd_info() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_lightd_info(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_latest_block() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_latest_block(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_out_of_bounds() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_out_of_bounds(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_nullifiers() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_nullifiers(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_nullifiers() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_nullifiers(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_nullifiers_reverse() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_nullifiers_reverse(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_lower() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_lower(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_upper() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_upper(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_reverse() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_reverse(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_block_range_out_of_bounds() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_block_range_out_of_bounds(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_transaction() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_transaction(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[ignore = "incomplete"]
#[tokio::test]
async fn send_transaction() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::send_transaction(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_taddress_txids_all() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_taddress_txids_all(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_taddress_txids_lower() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_taddress_txids_lower(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_taddress_txids_upper() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_taddress_txids_upper(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_taddress_balance() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_taddress_balance(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_taddress_balance_stream() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_taddress_balance_stream(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_mempool_tx() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_mempool_tx(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_mempool_stream() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_mempool_stream(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_tree_state_by_height() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_tree_state_by_height(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_tree_state_by_hash() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_tree_state_by_hash(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_tree_state_out_of_bounds() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_tree_state_out_of_bounds(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_latest_tree_state() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_latest_tree_state(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

// this is not a satisfactory test for this rpc and will return empty vecs.
// this rpc should also be tested in testnet/mainnet or a local chain with at least 2 shards should be cached.
#[tokio::test]
async fn get_subtree_roots_sapling() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_subtree_roots_sapling(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

// this is not a satisfactory test for this rpc and will return empty vecs.
// this rpc should also be tested in testnet/mainnet or a local chain with at least 2 shards should be cached.
#[tokio::test]
async fn get_subtree_roots_orchard() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_subtree_roots_orchard(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_all() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_all(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_lower() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_lower(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_upper() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_upper(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_out_of_bounds() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_out_of_bounds(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_stream_all() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_stream_all(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_stream_lower() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_stream_lower(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_stream_upper() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_stream_upper(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}

#[tokio::test]
async fn get_address_utxos_stream_out_of_bounds() {
    tracing_subscriber::fmt().init();

    zcash_local_net::test_fixtures::get_address_utxos_stream_out_of_bounds(
        ZCASHD_BIN.clone(),
        ZCASH_CLI_BIN.clone(),
        ZAINOD_BIN.clone(),
        LIGHTWALLETD_BIN.clone(),
    )
    .await;
}
