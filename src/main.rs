#[subxt::subxt(runtime_metadata_path = "artifacts/polkadot_metadata_full.scale")]
pub mod polkadot {}

use subxt::OnlineClient;
use subxt_lightclient::{ChainConfig, LightClient};
mod parachain_chainspec;
mod relaychain_chainspec;
use subxt::utils::AccountId32;
use subxt::PolkadotConfig;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let relay_chain_config = ChainConfig::chain_spec(relaychain_chainspec::CHAINSPEC);
    let chain_config = ChainConfig::chain_spec(parachain_chainspec::CHAINSPEC);

    let (light_client, _chain_rpc) = LightClient::relay_chain(relay_chain_config).unwrap();
    let chain_rpc = light_client.parachain(chain_config).unwrap();
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(chain_rpc.clone())
        .await
        .unwrap();
    sleep(Duration::from_secs(20)).await;

    let mut devices = api
        .storage()
        .at_latest()
        .await
        .unwrap()
        .iter(polkadot::storage().edge().devices_iter())
        .await
        .unwrap();
    let mut result = Vec::<(AccountId32, String)>::new();
    while let Some(Ok(x)) = devices.next().await {
        let r = x;
        let mut id = [0; 32];
        id.copy_from_slice(&r.key_bytes.as_slice()[48..]);
        let e = (id.into(), String::from_utf8(r.value.0).unwrap());
        result.push(e.clone());
        println!("OK {:?}", e);
    }
}
