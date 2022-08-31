"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const tslib_1 = require("tslib");
const express_1 = tslib_1.__importDefault(require("express"));
class JsInteract {
    constructor(port) {
        this.app = (0, express_1.default)();
        this.app.use(express_1.default.json());
        this.port = port;
        this.root();
        this.dummy();
        this.listen();
    }
    root() {
        this.app.get('/', (req, res) => {
            res.send('Hello js-interact');
        });
    }
    ;
    dummy() {
        this.app.post('/dummy', (req, res) => {
            const arg = req.body;
            console.log(arg);
            res.json(this.echo(arg.a, arg.b, arg.c));
        });
    }
    echo(a, b, c) {
        return { a, b, c };
    }
    listen() {
        this.server = this.app.listen(this.port, () => {
            console.log(`js-interact app listening on port ${this.port}`);
        });
    }
    close() {
        this.server.close();
    }
}
exports.default = JsInteract;
//# sourceMappingURL=JsInteract.js.map