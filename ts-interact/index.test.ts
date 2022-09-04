import request from 'supertest'
import Interact from './Interact'
import {homedir} from "os";
import {getConfig} from "./config";
import {connect, keyStores} from "near-api-js";
import InteractService from "./InteractService";

const keyStore = new keyStores.UnencryptedFileSystemKeyStore(`${homedir()}/.near-credentials`);
const nearConfig = getConfig("testnet", keyStore)

const near = await connect(nearConfig)
const account = await near.account("happyhappy.testnet")

const interactService = new InteractService(near, account);

const interact = new Interact(3000, interactService);

describe('Test the root path', () => {
  test('It should response the GET method /', async () => {
    const response = await request(interact.app).get('/');
    expect(response.statusCode).toBe(200);
  });
});

interact.close();
