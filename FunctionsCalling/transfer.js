const { DirectSecp256k1HdWallet } = require('@cosmjs/proto-signing');
const { GasPrice, calculateFee } = require('@cosmjs/stargate');
const { SigningCosmWasmClient } = require('@cosmjs/cosmwasm-stargate');

const fs = require('fs');

const sender = {
    mnemonic:
      'crane write limit match possible expand parade nice where slush hobby seven unusual kiwi wild mule famous false better wide cheese hire obvious tired',
    address: 'tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx',
    path: "m/44'/1'/0'/0/0'",
  };
const contract_address="tp15yldac6g05cuy0yc5grhtfuvw24vlhttp362ww0yuk3g0pt7lvss4822je"

async function main() {
  const recipient = 'tp1svpk6xpnyef6vxn7cvcs6mcghwmh3gpy0yy37q';

  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(sender.mnemonic, {
    prefix: 'tp',
  });

  const rpcEndpoint = 'https://rpc.test.provenance.io:443';
  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    wallet,
    { prefix: 'tp', gasPrice: '3000nhash' }
  );





  const gasPrice = GasPrice.fromString('3000nhash');
  const executeFee = calculateFee(4000000, gasPrice);
  const result = await client.execute(
    sender.address,
    contract_address,
    {
      transfer: {
        reciever:recipient,
         amount: "100",
         countrycode:"91"

      },
    },
    executeFee
  );

  // const wasmEvent = result.logs[0].events.find((e) => e.type === 'wasm');
  // console.log(
  //   'The `wasm` event emitted by the contract execution:',
  //   wasmEvent
    
  // );
  const hash=result.transactionHash
console.log("Transaction hash : ",hash)

  
}

main();