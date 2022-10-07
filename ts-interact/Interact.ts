import express from 'express';
import http from 'http';
import InteractService from './InteractService.js';

class Interact {
  public app: express.Application;
  public port: number;
  public server: http.Server;
  public interactService: InteractService;

  constructor(port: number, interactService: InteractService) {
    this.app = express();
    this.app.use(express.json());
    this.port = port;
    this.interactService = interactService;
    this.root();
    this.listen();
  }

  public root() {
    this.app.get('/', (req: express.Request, res: express.Response) => {
      res.send('Hello ts-interact');
    });

    this.app.get('/block', (req: express.Request, res: express.Response) => {
      this.interactService
        .getCurrentHeight()
        .then(height => res.json(height))
        .catch(err => {
          console.error(err);
          res.status(500).send(err);
        });
    });

    this.app.get(
      '/block/:id',
      (req: express.Request, res: express.Response) => {
        this.interactService
          .getBlock(parseInt(req.params.id))
          .then(block => res.json(block))
          .catch(err => {
            console.error(err);
            res.status(500).send(err);
          });
      }
    );

    this.app.get(
      '/account/:id',
      (req: express.Request, res: express.Response) => {
        this.interactService
          .queryAccount(req.params.id)
          .then(accountView => res.json(accountView))
          .catch(err => {
            console.error(err);
            res.status(500).send(err);
          });
      }
    );

    this.app.post(
      '/contract/:id/:method',
      (req: express.Request, res: express.Response) => {
        this.interactService
          .viewContractState(req.params.id, req.params.method, req.body)
          .then(result => res.json(result))
          .catch(err => {
            console.error(err);
            res.status(500).send(err);
          });
      }
    );

    this.app.put(
      '/contract/:id/:method',
      (req: express.Request, res: express.Response) => {
        this.interactService
          .changeContractState(req.params.id, req.params.method, req.body)
          .then(result => res.send(result))
          .catch(err => {
            console.error(err);
            res.status(500).send(err);
          });
      }
    );
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

export default Interact;
