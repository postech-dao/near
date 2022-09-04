export function getConfig(env, keyStore) {
    switch (env) {
        case 'production':
        case 'mainnet':
            return {
                headers: { "url": "https://rpc.mainnet.near.org" },
                networkId: 'mainnet',
                nodeUrl: 'https://rpc.mainnet.near.org',
                walletUrl: 'https://wallet.near.org',
                helperUrl: 'https://helper.mainnet.near.org',
                keyStore: keyStore
            };
        case 'development':
        case 'testnet':
            return {
                headers: { "url": "https://rpc.testnet.near.org" },
                networkId: 'testnet',
                nodeUrl: 'https://rpc.testnet.near.org',
                walletUrl: 'https://wallet.testnet.near.org',
                helperUrl: 'https://helper.testnet.near.org',
                keyStore: keyStore
            };
        default:
            throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`);
    }
}
//# sourceMappingURL=config.js.map