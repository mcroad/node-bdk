#![deny(clippy::all)]

use napi::bindgen_prelude::ToNapiValue;
use bdk;

#[macro_use]
extern crate napi_derive;

// can't reexport enum, must reimplement
// https://github.com/napi-rs/napi-rs/issues/1463
#[napi]
pub enum Network {
  Bitcoin,
  Regtest,
  Signet,
  Testnet,
}
impl From<Network> for bdk::bitcoin::Network {
  fn from(item: Network) -> Self {
    match item {
      Network::Bitcoin => bdk::bitcoin::Network::Bitcoin,
      Network::Regtest => bdk::bitcoin::Network::Regtest,
      Network::Signet => bdk::bitcoin::Network::Signet,
      Network::Testnet => bdk::bitcoin::Network::Testnet,
    }
  }
}

#[napi]
pub struct ElectrumBlockchain {
  inner: bdk::blockchain::ElectrumBlockchain,
}

#[napi]
impl ElectrumBlockchain {
  #[napi(constructor)]
  pub fn new(url: String) -> Self {
    ElectrumBlockchain {
      inner: bdk::blockchain::ElectrumBlockchain::from(
        bdk::electrum_client::Client::new(&url).unwrap(),
      ),
    }
  }
}

#[napi]
pub struct Wallet {
  inner: bdk::Wallet<bdk::database::MemoryDatabase>,
}

#[napi]
impl Wallet {
  #[napi(constructor)]
  pub fn new(descriptor: String, change_descriptor: Option<String>, network: Network) -> Self {
    Wallet {
      inner: bdk::Wallet::new(
        descriptor.as_str(),
        change_descriptor.as_ref().map(|s| s.as_str()),
        network.into(),
        bdk::database::MemoryDatabase::default(),
      )
      .unwrap(),
    }
  }

  #[napi]
  pub fn sync(&self, chain: &ElectrumBlockchain) -> () {
    self
      .inner
      .sync(&chain.inner, bdk::SyncOptions::default())
      .unwrap();
  }

  #[napi]
  pub fn get_balance(&self) -> Balance {
    Balance {
      inner: self.inner.get_balance().unwrap(),
    }
  }
}

#[napi]
pub struct Balance {
  inner: bdk::Balance,
}

#[napi]
impl Balance {
  #[napi]
  pub fn get_spendable(&self) -> u64 {
    self.inner.get_spendable()
  }

  #[napi]
  pub fn get_total(&self) -> u64 {
    self.inner.get_total()
  }
}
