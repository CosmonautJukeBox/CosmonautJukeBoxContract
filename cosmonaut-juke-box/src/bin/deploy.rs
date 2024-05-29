use cosmonaut_juke_box::{
    interface::CosmonautContract, msg::InstantiateMsg, CosmonautExecuteMsgFns, CosmonautQueryMsgFns,
};
use cw_orch::environment::{ChainKind, NetworkInfo};
use cw_orch::{anyhow, prelude::*};

pub const NEUTRON_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "neutron",
    pub_address_prefix: "neutron",
    coin_type: 118u32,
};

/// <https://github.com/cosmos/chain-registry/blob/master/testnets/neutrontestnet/chain.json>
pub const PION_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "pion-1",
    gas_denom: "untrn",
    gas_price: 0.075,
    grpc_urls: &["http://grpc-palvus.pion-1.ntrn.tech:80"],
    network_info: NEUTRON_NETWORK,
    lcd_url: Some("https://rest-palvus.pion-1.ntrn.tech"),
    fcd_url: None,
};

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";
pub fn main() -> anyhow::Result<()> {
    //std::env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
    // ANCHOR: chain_construction
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    pretty_env_logger::init(); // Used to log contract and chain interactions

    // The network to deploy the contract on
    let network = networks::LOCAL_JUNO;

    let network = PION_1;
    // The chain to deploy the contract on
    let chain = DaemonBuilder::default().chain(network).build()?;

    let contract = CosmonautContract::new(chain);

    contract.upload()?;
    contract.instantiate(&InstantiateMsg {}, None, None)?;

    Ok(())
}
