// csv parser 
import csv from 'csv-parser';
import fs, { PathLike } from 'fs';

// utils
import { utils } from 'near-api-js';

export type Participant = {
    account: String,
    amount: String,
};

export const parseCSVWithChunks = async (path: PathLike, chunkSize: number): Promise<Participant[][]> => {
    if (chunkSize === 0) throw new Error("Can not chunk arrray by zero");
    const results: Participant[] = await parseCSV(path);
    
    // then, chunk array with preferred size
    let chunks: Participant[][] = [];
    for (let i = 0; i < results.length; i += chunkSize) {
        const chunk = results.slice(i, i + chunkSize);
        chunks.push(chunk);
    }
    // console.log("chunks", chunks);

    return chunks;
}

export const parseCSV = async (path: PathLike): Promise<Participant[]> => {
    // store all formatted data in results
    let results: Participant[] = [];

    const parseConfig = csv({
        headers: ["account", "amount"],
        mapHeaders: ({ header, index }) => header.toLowerCase(),
        skipLines: 1,
    });
    const readStream = fs.createReadStream(path).pipe(parseConfig);
    // readStream.on('end', () => console.log(results));
    
    for await (const data of readStream) {
        results.push({
            ...data,
            amount: utils.format.parseNearAmount(data.amount)
        })
    }


    return results;
}