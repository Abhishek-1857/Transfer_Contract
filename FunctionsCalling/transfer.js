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
  const sender2 = {
    mnemonic:
      'brass eye desert inflict note focus ladder alcohol illness bless width exhibit faculty jelly flee estate disease sick typical chief type truth puppy kick',
    address: 'tp1ua9fz57n0upam2l40ydj7vn46v8snpfgxyffj9',
    path: "m/44'/1'/0'/0/0'",
  };
const contract_address="tp1ysyf7ypd2vfn74a8d4dfexmqcy4xu9zu30jxj6pu0r94f88ehe3sgegp5l"

async function main() {
  const recipient = 'tp1ua9fz57n0upam2l40ydj7vn46v8snpfgxyffj9';
console.log("Herrrrrrrrrrrreeeeeeeeeee")
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(sender.mnemonic, {
    prefix: 'tp',
  });
  console.log("Herrrrrrrrrrrreeeeeeeeeee")

  const rpcEndpoint = 'https://rpc.test.provenance.io:443';
  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    wallet,
    { prefix: 'tp', gasPrice: '3000nhash' }
  );
  console.log("Herrrrrrrrrrrreeeeeeeeeee")





  const gasPrice = GasPrice.fromString('3000nhash');
  const executeFee = calculateFee(4000000, gasPrice);
  console.log("Herrrrrrrrrrrreeeeeeeeeee")

  const result = await client.execute(
    sender.address,
    contract_address,
    {
      // transfer: {
      //   reciever:recipient,
      //    amount: "100",
      //    countrycode:"91"

      // },
      // remove_shareholder: {
      //   account:recipient,
      // },
      "create": {
        "supply": "100000",
        "denom": "RCustomMarker"
    }
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