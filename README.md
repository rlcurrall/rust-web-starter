## Local Development

### Setup
First, you must generate the private and public PEM keys which are used by the
server to authenticate API requests using a bearer token. To generate these keys
simply run:

```bash
./scripts/dev-setup
```

This will generate 3 files:

* _conf/public.pem_ - the public key for validating JWT tokens
* _conf/private.pem_ - the private key for creating JWT tokens
* _jwt.txt_ - a JWT token with a 1 hour expiration for local testing

If you need a new JWT token at any time you can simply run:

```bash
./scripts/jwt new user@example.com conf/private.pem
```

### Starting the Server
Once setup is complete, you can run:

```bash
docker compose up -d
```

This will start the database and application server as docker containers. The docker compose
stack uses a development container for the app server which is not optimized for production
but will perform automatic rebuilds whenever you modify any of the source files.
