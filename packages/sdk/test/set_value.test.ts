import { Connection, Keypair, TransactionMessage, VersionedTransaction } from '@solana/web3.js';
import { JsonMetadata, createInitializeInstruction, createSetValueInstruction, find_metadata_account } from '../src';
import test from 'tape';

test('SetValue', async (t) => {
    let connection = new Connection("http://localhost:8899", 'finalized');
    let payer = Keypair.generate();
    const airdrop_txid = await connection.requestAirdrop(payer.publicKey, 1000000000);
    await connection.confirmTransaction(airdrop_txid, 'finalized');
    console.log(airdrop_txid);
    let jsonAccountKeypair = Keypair.generate();
    let jsonMetadataAccount = find_metadata_account(jsonAccountKeypair.publicKey);

    let init_ix = createInitializeInstruction({
        jsonAccount: jsonAccountKeypair.publicKey,
        jsonMetadataAccount: jsonMetadataAccount[0],
        payer: payer.publicKey,
    });

    let set_value_ix_0 = createSetValueInstruction({
        jsonAccount: jsonAccountKeypair.publicKey,
        jsonMetadataAccount: jsonMetadataAccount[0],
        payer: payer.publicKey,
    },
    {
        setValueArgs: {
            value: '{"name": "Breadron"}'
        },
    });

    let set_value_ix_1 = createSetValueInstruction({
        jsonAccount: jsonAccountKeypair.publicKey,
        jsonMetadataAccount: jsonMetadataAccount[0],
        payer: payer.publicKey,
    },
    {
        setValueArgs: {
            value: '{"external_url": "https://www.breadheads.io/"}'
        },
    });

    let latestBlockhash = await connection.getLatestBlockhash();
    const msg = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: latestBlockhash.blockhash,
        instructions: [init_ix, set_value_ix_0, set_value_ix_1],
    }).compileToV0Message();
    const tx = new VersionedTransaction(msg);

    tx.sign([payer, jsonAccountKeypair]);
    const txid = await connection.sendTransaction(tx, {skipPreflight: true});
    await connection.confirmTransaction(txid, 'finalized');
    console.log(txid);

    const jsonAccountInfo = await connection.getAccountInfo(jsonAccountKeypair.publicKey);
    const jsonAccountData = jsonAccountInfo.data.toString();
    console.log(jsonAccountData);

    const jsonMetadataAccountData = await JsonMetadata.fromAccountAddress(connection, jsonMetadataAccount[0]);
    console.log(jsonMetadataAccountData);
    t.assert(jsonMetadataAccountData.bump == jsonMetadataAccount[1], "bump is correct");
    t.assert(jsonMetadataAccountData.mutable == true, "Account is mutable");
    let authorities = jsonMetadataAccountData.authorities.map((authority) => authority.toString());
    console.log(authorities);
    console.log(payer.publicKey);
    t.assert(authorities.length == 1, "There is one authority");
    t.assert(authorities.includes(payer.publicKey.toString()), "The only authority is the payer");
});
