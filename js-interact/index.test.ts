import request from 'supertest'
import JsInteract from './JsInteract'


const jsInteract = new JsInteract(3000);

describe('Test the root path', () => {
  test('It should response the GET method /', async () => {
    const response = await request(jsInteract.app).get('/');
    expect(response.statusCode).toBe(200);
  });
});


describe('Test dummy json api', () => {
    test('It should response the POST method /dummy', async () => {
      const response = await request(jsInteract.app).post('/dummy')
        .set('Accept', 'application/json')
        .send({ a: 123, b: 'teststring', c:[1,2,3] });
      expect(response.statusCode).toBe(200);
      expect(response.headers['content-type']).toMatch('/json');
      expect(response.headers['content-type']).toMatch('/json');
      expect(response.body).toEqual({ a: 123, b: 'teststring', c:[1,2,3] });
    });
});


jsInteract.close();