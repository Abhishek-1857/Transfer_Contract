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

async function main() {
  try {
    // const recipient = 'wasm14u97wl8dravh9l72jt7p763zccxlwyzl9kuudl';

    const wallet = await DirectSecp256k1HdWallet.fromMnemonic(sender.mnemonic, {
      prefix: 'tp',
    });
    const [firstAccount] = await wallet.getAccounts();
    const rpcEndpoint = 'https://rpc.test.provenance.io:443';
    const client = await SigningCosmWasmClient.connectWithSigner(
      rpcEndpoint,
      wallet,
      { prefix: 'tp', gasPrice: '3000nhash' }
    );

    // console.log(await (await (client.getBlock(3599))).txs[0])

    // Upload contract
    const gasPrice = GasPrice.fromString('3000nhash');
    const wasm = fs.readFileSync('./target/wasm32-unknown-unknown/release/transfer.wasm');
    console.log('Got it', Boolean(wasm));
    const uploadFee = calculateFee(4000000, gasPrice);
    const uploadReceipt = await client.upload(
      sender.address,
      wasm,
      uploadFee,
      'Upload hackatom contract'
    );
    console.info('Upload succeeded. Receipt:', uploadReceipt);

    // Instantiate
    const instantiateFee = calculateFee(500000, gasPrice);
    const msg = {
      name: 'LEMMA',
      symbol: 'LMT',
      max_supply:'10000000000',
      initial_balances: [
        {
          address: sender.address,
          amount: '100000',
          freeze_amount:'100'
        },
      ],
      share_holders: ["tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx"],
      authorised_countries:['91'],
      max_hold_balance: '10000',
      
    };
    const contract = await client.instantiate(
      sender.address,
      uploadReceipt.codeId,
      msg,
      'My instance',
      instantiateFee,
      { memo: `Create a hackatom instance` }
    );
    console.info(
      `Contract instantiated at: `,
      contract,
      'address::',
      contract.contractAddress
    );

   
  } catch (er) {
    console.log('I am error ', er);
  }
}

main();
