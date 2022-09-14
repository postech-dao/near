import express from "express";
import http from "http";

class TsInteract {
  public app: express.Application;
  public port: number;
  public server: http.Server;

  constructor(port: number) {
    this.app = express();
    this.app.use(express.json());
    this.port = port;
    this.root();
    this.listen();
  }

  public root() {
    this.app.get("/", (req: express.Request, res: express.Response) => {
      res.send("Hello ts-interact");
    });
  }

  public listen() {
    this.server = this.app.listen(this.port, () => {
      console.log(`ts-interact app listening on port ${this.port}`);
    });
  }

  public close() {
    this.server.close();
  }
}

export default TsInteract;
