import test from 'ava';

import { Balance, ElectrumBlockchain, Wallet, Network } from '../index.js';

const wallet = new Wallet(
  'wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)',
  'wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)',
  Network.Testnet,
);

test('check balance', (t) => {
  const chain = new ElectrumBlockchain('ssl://electrum.blockstream.info:60002');

  wallet.sync(chain);

  const balance = wallet.getBalance();

  t.assert(balance.getTotal(), 'has a balance');
  t.assert(balance.getSpendable(), 'has a spendable balance');
});
