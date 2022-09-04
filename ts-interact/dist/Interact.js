import express from "express";
class Interact {
    constructor(port, interactService) {
        this.app = express();
        this.app.use(express.json());
        this.port = port;
        this.interactService = interactService;
        this.root();
        this.listen();
    }
    root() {
        this.app.get("/", (req, res) => {
            res.send("Hello ts-interact");
        });
        this.app.get("/block", (req, res) => {
            this.interactService.getCurrentHeight()
                .then((height) => res.json(height))
                .catch(err => {
                console.error(err);
                res.status(500).send(err);
            });
        });
        this.app.get("/block/:id", (req, res) => {
            this.interactService.getBlock(parseInt(req.params.id))
                .then((block) => res.json(block))
                .catch(err => {
                console.error(err);
                res.status(500).send(err);
            });
        });
        this.app.get("/account/:id", (req, res) => {
            this.interactService.queryAccount(req.params.id)
                .then((accountView) => res.json(accountView))
                .catch(err => {
                console.error(err);
                res.status(500).send(err);
            });
        });
        this.app.post("/contract/:id/:method", (req, res) => {
            this.interactService.viewContractState(req.params.id, req.params.method, req.body)
                .then((result) => res.json(result))
                .catch(err => {
                console.error(err);
                res.status(500).send(err);
            });
        });
        this.app.put("/contract/:id/:method", (req, res) => {
            this.interactService.changeContractState(req.params.id, req.params.method, req.body)
                .then((result) => res.send(result))
                .catch(err => {
                console.error(err);
                res.status(500).send(err);
            });
        });
    }
    listen() {
        this.server = this.app.listen(this.port, () => {
            console.log(`ts-interact app listening on port ${this.port}`);
        });
    }
    close() {
        this.server.close();
    }
}
export default Interact;
//# sourceMappingURL=Interact.js.map