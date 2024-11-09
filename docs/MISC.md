# Misc

## Conflux Core Space Differences with Ethereum

1. **Address Format**: Conflux uses a different address format than Ethereum. Conflux uses a base32 encoded address, check details [here](https://doc.confluxnetwork.org/docs/core/core-space-basics/addresses)
2. **Transaction Structure**: Conflux uses a different transaction structure than Ethereum, check details [here](https://doc.confluxnetwork.org/docs/core/core-space-basics/transactions/tx-fields)
3. Bip32 Derivation Path: Conflux coin type is 503, while Ethereum coin type is 60. So the derivation path for Conflux is `m/44'/503'/0'/0/0`.