import request from 'supertest'
import TsInteract from './TsInteract'


const tsInteract = new TsInteract(3000);

describe('Test the root path', () => {
  test('It should response the GET method /', async () => {
    const response = await request(tsInteract.app).get('/');
    expect(response.statusCode).toBe(200);
  });
});

tsInteract.close();
