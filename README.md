# README

## Gitpod

[Gitpod nextjs-neardev-rust-template](https://gitpod.io/#https://github.com/Eigo-Mt-Fuji/nextjs-neardev-rust-template)

## Environments

- OS: macOS(>=10.15)
- NodeJs>=16.16
- Rust>=1.62
- Terraform 1.2

### Last  

| Name | Version |
|:----:|:----:|
| OS | macOS 10.15.5(Intel Processor)  | 
| nodejs | 16.16.0  | 
| rust | 1.62.1 |  
| terraform | 1.2.7  | 

## How to use

### Smart contract build and deploy

- Build

```
npm run build:contract
```

- Deploy SmartContract to testnet

```
npm run dev:deploy:contract
```

### Prepare Frontend(next.js) .env.local

- Open contracts/neardev/dev-account.env and check contract name

```
$ cat contracts/neardev/dev-account.env 
CONTRACT_NAME=dev-1660474152460-14596747053304
```

- Put .env.local file for next.js

```
touch .env.local
cat <<EOF frontend/.env.local
NEXT_PUBLIC_CONTRACT_NAME=dev-1660474152460-14596747053304
EOF
```

### Frontend(next.js) build 

- Build

```
npm run build:frontend
```

### Frontend(next.js) preview(local)

- Start Dev Frontend Server

```
npm run dev
```

### Frontend deploy(test)

- Ensure your serverless framework access key `$SERVERLESS_ACCESS_KEY` is exists
  - if does not exist.
    - login app.serverless.com and publish your access key
      - [access here](https://app.serverless.com/efgriver/settings/accessKeys)
        - don't forget to put secret(or ensure) environment variable on repo for test
          - [access here](https://github.com/Eigo-Mt-Fuji/awesome-rust-dapp/settings/environments/594355632/edit)

```
echo $SERVERLESS_ACCESS_KEY
```

- Execute serverless deploy

```
$ npm run deploy:serverless:test
```

- Ensure serverless deploy is done(messages should be shown like following)

```
> awesome-rust-dapp@0.1.0 deploy:serverless:test
> cp serverless.test.yml serverless.yml && components-v1 && rm -rf serverless.yml


  awesomeRustDapp: 
    appUrl:         https://awesome-rust-dapp.efgriver.com
    bucketName:     v2j8wjn-k5ncpd8
    distributionId: E1WHCQULHESL5I

  287s › awesomeRustDapp › done
```

### Unit test(rust)

```
npm run test:unit
```

### Integration test(next.js / cypress)

- start dev server

```
npm run dev
```

- open another terminal

- start integration test
  - after cypress launched successfully, choose E2E and run each test spec manually.

```
npm run test:integration:ts
```

### Deploy frontend app AWS(test)


```
NEXT_PUBLIC_CONTRACT_NAME=dev-1660474152460-14596747053304
SERVERLESS_ACCESS_KEY=my secret key
SLS_STAGE=test
```

- try deploy serverless deploy

```
export SERVERLESS_ACCESS_KEY=my secret key
source frontend/.env.local
npm run deploy:serverless
```
