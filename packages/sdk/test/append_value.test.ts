import { Connection, Keypair, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import {
  JsonMetadata,
  createInitializeInstruction,
  createSetValueInstruction,
  find_metadata_account,
} from '../src';
import test from 'tape';

// const test_json_chunks = [
//     `{
//         "name": "SMB #2163",
//         "symbol": "SMB",
//         "description": "SMB is a collection of 5000 randomly generated 24x24 pixels NFTs on the Solana Blockchain. Each SolanaMonkey is unique and comes with different type and attributes varying in rarity.",
//         "seller_fee_basis_points": 600,
//         "image": "https://arweave.net/wnNcb1q_NbuytZcD6_DCJQ-szC5gu-N4ka51CKtbfTk",
//         "external_url": "https://solanamonkey.business/",
//         "collection": {
//             "name": "SMB Gen2",
//             "family": "SMB"
//         },
//         "attributes": [
//         {
//             "trait_type": "Attributes Count",
//             "value": 2
//         },
//         {
//             "trait_type": "Type",
//             "value": "Brown"
//         },
//         {
//             "trait_type": "Clothes",
//             "value": "None"
//         },
//         {
//             "trait_type": "Ears",
//             "value": "None"
//         },
//         {
//             "trait_type": "Mouth",
//             "value": "None"
//         },
//         {
//             "trait_type": "Eyes",
//             "value": "Purple Glasses"
//         },
//         {
//             "trait_type": "Hat",
//             "value": "Protagonist White Hat"
//         }
//         ],
//         "properties": {
//         "files": [
//             {
//                 "uri": "https://arweave.net/wnNcb1q_NbuytZcD6_DCJQ-szC5gu-N4ka51CKtbfTk",
//                 "type": "image/png"
//             },
//             {
//                 "uri": "https://cdn.solanamonkey.business/gen2/2163.png",
//                 "type": "image/png",
//                 "cdn": true
//             }
//         ],
//         "category": "image",
//         "creators": [
//             {
//                 "address": "9uBX3ASjxWvNBAD1xjbVaKA74mWGZys3RGSF7DdeDD3F",
//                 "verified": true,
//                 "share": 100
//             }
//         ]
//         }
//     }`,
// ];

test('AppendValue', async (t) => {
  const connection = new Connection('http://localhost:8899', 'finalized');
  const payer = Keypair.generate();
  const airdrop_txid = await connection.requestAirdrop(payer.publicKey, 1000000000);
  await connection.confirmTransaction(airdrop_txid, 'finalized');
  console.log(airdrop_txid);
  const jsonAccountKeypair = Keypair.generate();
  const jsonMetadataAccount = find_metadata_account(jsonAccountKeypair.publicKey);

  const init_ix = createInitializeInstruction({
    jsonAccount: jsonAccountKeypair.publicKey,
    jsonMetadataAccount: jsonMetadataAccount[0],
    payer: payer.publicKey,
  });

  const set_value_ix_0 = createSetValueInstruction(
    {
      jsonAccount: jsonAccountKeypair.publicKey,
      jsonMetadataAccount: jsonMetadataAccount[0],
      payer: payer.publicKey,
    },
    {
      setValueArgs: {
        value: '{"name": "Bread On-Chain", "symbol": "BREAD"}',
      },
    },
  );

  const set_value_ix_1 = createSetValueInstruction(
    {
      jsonAccount: jsonAccountKeypair.publicKey,
      jsonMetadataAccount: jsonMetadataAccount[0],
      payer: payer.publicKey,
    },
    {
      setValueArgs: {
        value:
          '{"description": "A bread! But on-chain!", "seller_fee_basis_points": 500, "external_url": "https://breadheads.io"}',
      },
    },
  );

  const latestBlockhash0 = await connection.getLatestBlockhash();
  const msg0 = new TransactionMessage({
    payerKey: payer.publicKey,
    recentBlockhash: latestBlockhash0.blockhash,
    instructions: [init_ix, set_value_ix_0, set_value_ix_1],
  }).compileToV0Message();
  const tx0 = new VersionedTransaction(msg0);

  tx0.sign([payer, jsonAccountKeypair]);
  const txid0 = await connection.sendTransaction(tx0, { skipPreflight: true });
  await connection.confirmTransaction(txid0, 'finalized');
  console.log(txid0);

  const jsonAccountInfo0 = await connection.getAccountInfo(jsonAccountKeypair.publicKey);
  const jsonAccountData0 = jsonAccountInfo0.data.toString();
  console.log('JSON: ', jsonAccountData0);

  const jsonMetadataAccountData = await JsonMetadata.fromAccountAddress(
    connection,
    jsonMetadataAccount[0],
  );
  console.log(jsonMetadataAccountData);
  t.assert(jsonMetadataAccountData.bump == jsonMetadataAccount[1], 'bump is correct');
  t.assert(jsonMetadataAccountData.mutable == true, 'Account is mutable');
  const authorities = jsonMetadataAccountData.authorities.map((authority) => authority.toString());
  console.log(authorities);
  console.log(payer.publicKey);
  t.assert(authorities.length == 1, 'There is one authority');
  t.assert(authorities.includes(payer.publicKey.toString()), 'The only authority is the payer');

  const set_value_ix_2 = createSetValueInstruction(
    {
      jsonAccount: jsonAccountKeypair.publicKey,
      jsonMetadataAccount: jsonMetadataAccount[0],
      payer: payer.publicKey,
    },
    {
      setValueArgs: {
        value:
          '{"image": "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAMAAABEpIrGAAAAPFBMVEUAAADoijZnOTEODAyyi3j/4Lfisn78pXCwWyz4xTr////icoX49kRhpT9Hcji0ICpCpFk9b0MUNGQoXMSCKWhnAAAAAXRSTlMAQObYZgAAAP9JREFUOMuNkAmugzAMRDHO4iwQaO9/1z9Oot8FI3UQGYt5tiOW3xRcwHmcztm5C0ocxxFsAp+BnOcBc/YAPLWeQf3mBkFqrbC7OzgRqbpp7boSpYigv+cW00opo98mWmtl9tvEo7XRTxoQ4f0Eno/eT5Ag6f454enczFW9Wr//lDZumbry9gJeWmljRkLD1/UKZOZdoJ0503XAQrQj78ROAC7SO+KdZgPMPA4b4DcZwExEZnFDSM5i5lBkwvKcCcbRAHykf0VvAMn7wUQUyQAopegj5GNKZK9IYCIOe4X3mE1ItbAAfEU7c4paLYY8pIC6lWPEXGEPgHQ2EeL3/A89Zgj/88QojwAAAABJRU5ErkJggg=="}',
      },
    },
  );

  const latestBlockhash1 = await connection.getLatestBlockhash();
  const msg1 = new TransactionMessage({
    payerKey: payer.publicKey,
    recentBlockhash: latestBlockhash1.blockhash,
    instructions: [set_value_ix_2],
  }).compileToV0Message();
  const tx1 = new VersionedTransaction(msg1);

  tx1.sign([payer]);
  const txid1 = await connection.sendTransaction(tx1, { skipPreflight: true });
  await connection.confirmTransaction(txid1, 'finalized');
  console.log(txid1);

  const jsonAccountInfo1 = await connection.getAccountInfo(jsonAccountKeypair.publicKey);
  const jsonAccountData1 = jsonAccountInfo1.data.toString();
  console.log('JSON: ', jsonAccountData1);
});
