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
    this.dummy();
    this.listen();
  }

  public root() {
    this.app.get("/", (req: express.Request, res: express.Response) => {
      res.send("Hello ts-interact");
    });
  }

  public dummy() {
    this.app.post("/dummy", (req: express.Request, res: express.Response) => {
      const arg = req.body;
      console.log(arg);
      res.json(this.echo(arg.a, arg.b, arg.c));
    });
  }

  private echo(a: number, b: string, c: number[]) {
    return { a, b, c };
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
