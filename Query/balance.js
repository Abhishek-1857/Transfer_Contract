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

const contract_address="tp15yldac6g05cuy0yc5grhtfuvw24vlhttp362ww0yuk3g0pt7lvss4822je"
const address1="tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx"
const address2="tp1svpk6xpnyef6vxn7cvcs6mcghwmh3gpy0yy37q"

const main = async () => {
  const client = await SigningCosmWasmClient.connect(rpcUrl);

  console.log(
    ' VALUE ::::',
    await client.queryContractSmart(
      contract_address,
      {
        balance: { address: address1 },
      }
    )
  );
  console.log(
    ' VALUE ::::',
    await client.queryContractSmart(
      contract_address,
      {
        balance: { address: address2 },
      }
    )
  );
};

main();
