const util = require('util');
// const atob = require('atob');
const {
  SigningCosmWasmClient,
  fromBinary,
  createDefaultRegistry,
  isMsgExecuteEncodeObject,
} = require('@cosmjs/cosmwasm-stargate');
const { sha256 } = require('@cosmjs/crypto');
const { fromBase64, toHex, toBase64 } = require('@cosmjs/encoding');
const {
  DirectSecp256k1HdWallet,
  decodeTxRaw,
  Registry,
} = require('@cosmjs/proto-signing');
const {
  assertIsDeliverTxSuccess,
  assertIsBroadcastTxSuccess,
  SigningStargateClient,
  StdFee,
  calculateFee,
  GasPrice,
  coins,
  defaultRegistryTypes,
} = require('@cosmjs/stargate');

const rpcUrl = 'https://rpc.test.provenance.io:443';

const contract_address="tp1h82y6m2rhmf7plggdt3smaxj3cth5ffu9dg7fpenxxjzpkjlzc4se0zz59"
const address1="tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx"
const address2="tp1ua9fz57n0upam2l40ydj7vn46v8snpfgxyffj9"

const main = async () => {
  const client = await SigningCosmWasmClient.connect(rpcUrl);

  console.log(
    ' VALUE ::::',
    await client.queryContractSmart(
      contract_address,
      {
        share_holders: {},
       
      },
    )
  );
  console.log(
    ' VALUE ::::',
    await client.queryContractSmart(
      contract_address,
      {
        frozen_balance: { address: "tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx" },
      }
    )
  );
};

main();
