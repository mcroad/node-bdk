# node-bdk

### EXPERIMENTAL. DO NOT USE.

Use natively compiled [BDK](https://github.com/bitcoindevkit/bdk) in a nodejs context with TypeScript typings. Extremely experimental. Uses https://napi.rs/ to generate bindings.

Inspired by [bdk-nodejs](https://github.com/danielabrozzoni/bdk-nodejs) and a week at [Bitcoin Park](https://bitcoinpark.co/).

## Supported features

- [x] Create a wallet
- [x] Sync to Electrum
- [x] Get balance

## Example

```ts
import { ElectrumBlockchain, Wallet, Network } from 'bdk';

const wallet = new Wallet(
  'wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)',
  'wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)',
  Network.Testnet,
);

const chain = new ElectrumBlockchain('ssl://electrum.blockstream.info:60002');
wallet.sync(chain);

const balance: bigint = wallet.getBalance();
console.log('balance', balance); // 5748838n
```
