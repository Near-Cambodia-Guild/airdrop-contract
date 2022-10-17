import dotenv from 'dotenv';
dotenv.config();

import { PathLike } from "fs";
import { Near, Account, Contract, DEFAULT_FUNCTION_CALL_GAS } from "near-api-js";
import { nearConnection } from './connect';

import { Participant, parseCSVWithChunks } from './csv-utils';
const ACCOUNT_NAME = "wooh.testnet";
const CONTRACT_NAME = "airdrop.wooh.testnet";

const main = async (path: PathLike, chunkSize: number) => {
    // connect to NEAR
    const conn: Near = await nearConnection();
    const account: Account = await conn.account(ACCOUNT_NAME);

    const chunks: Participant[][] = await parseCSVWithChunks(path, chunkSize);

    const contract: any = new Contract(account, CONTRACT_NAME, {
        viewMethods: ["owner", "available_withdraw"],
        changeMethods: ["airdrop", "withdraw", "withdraw_all", "transfer_ownership"]
    });

    // chunks.forEach(async (participants: Participant[], _idx) => ;
    for await (const participants of chunks) {
        const txn = await contract.airdrop({
            args: { participants },
            gas: DEFAULT_FUNCTION_CALL_GAS * 10, // 300TGAS attached_gas
        });
        console.log(txn)
    }
}

main("./data.csv", 300)