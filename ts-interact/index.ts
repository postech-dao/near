import InteractService from './InteractService.js';
import Interact from './Interact.js';
import {homedir} from 'os';
import {getConfig} from './config.js';
import {connect, keyStores} from 'near-api-js';

const keyStore = new keyStores.UnencryptedFileSystemKeyStore(
  `${homedir()}/.near-credentials`
);
const nearConfig = getConfig('testnet', keyStore);

async function main() {
  const near = await connect(nearConfig);
  const account = await near.account('happyhappy.testnet');

  const interactService = new InteractService(near, account);

  new Interact(3000, interactService);
}

main();
