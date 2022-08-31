"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const tslib_1 = require("tslib");
const supertest_1 = tslib_1.__importDefault(require("supertest"));
const JsInteract_1 = tslib_1.__importDefault(require("./JsInteract"));
const jsInteract = new JsInteract_1.default(3000);
describe('Test the root path', () => {
    test('It should response the GET method', () => tslib_1.__awaiter(void 0, void 0, void 0, function* () {
        const response = yield (0, supertest_1.default)(jsInteract.app).get('/');
        expect(response.statusCode).toBe(200);
        jsInteract.close();
    }));
});
//# sourceMappingURL=index.test.js.map