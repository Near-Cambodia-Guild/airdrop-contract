import { keyStores, connect, ConnectConfig, Near } from "near-api-js";

// creates keyStore using private key in local storage
// const myKeyStore = new keyStores.BrowserLocalStorageKeyStore();

// creates a keyStore that searches for keys in .near-credentials
const homedir = require("os").homedir();
const CREDENTIALS_DIR = ".near-credentials";
const credentialsPath = require("path").join(homedir, CREDENTIALS_DIR);
const myKeyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

type Network = 'mainnet' | 'testnet' | 'localnet'; 
const ConnnectionConfig: {[network in Network]: ConnectConfig} = {
    'mainnet': {
        networkId: "mainnet",
        keyStore: myKeyStore, // first create a key store
        nodeUrl: "https://rpc.mainnet.near.org",
        walletUrl: "https://wallet.mainnet.near.org",
        helperUrl: "https://helper.mainnet.near.org",
        // explorerUrl: "https://explorer.mainnet.near.org",
    },
    'testnet': {
        networkId: "testnet",
        keyStore: myKeyStore, // first create a key store 
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        // explorerUrl: "https://explorer.testnet.near.org",
    },
    'localnet': {
        networkId: "local",
        nodeUrl: "http://localhost:3030",
        walletUrl: "http://localhost:4000/wallet",
    }
};

export const nearConnection = async (): Promise<Near> => {
    if (!process.env.NETWORK) throw Error("No NETWORK!");
    const connectionConfig = ConnnectionConfig[process.env.NETWORK as Network];
    if (!connectionConfig) throw Error("Given network is not supported!");
    return await connect(connectionConfig);
};