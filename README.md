# README

## Gitpod

[Gitpod awesome-rust-dapp](https://gitpod.io/#https://github.com/Eigo-Mt-Fuji/awesome-rust-dapp)

## Environments

- OS: macOS(>=10.15)
- NodeJs>=16.16
- Rust>=1.62

## How to use

- Build

```
npm run build
```

- Deploy SmartContract to testnet

```
npm run dev:deploy:contract
```

- Open neardev/dev-account.env and check contract name

```
$ cat neardev/dev-account.env 
CONTRACT_NAME=dev-1660474152460-14596747053304
```

- Put .env.local file for next.js

```
touch frontend/.env.local
cat <<EOF frontend/.env.local
NEXT_PUBLIC_CONTRACT_NAME=dev-1660474152460-14596747053304
EOF
```

- Start Dev Frontend Server

```
npm run dev
```

### Deploy frontend(test)

```
$ npm run build:frontend
```

```
$ npm run deploy:serverless:test
```

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

- login app.serverless.com and publish your access key
  - [access here](https://app.serverless.com/efgriver/settings/accessKeys)

- put secret(or ensure) environment variable on repo for test
  - [access here](https://github.com/Eigo-Mt-Fuji/awesome-rust-dapp/settings/environments/594355632/edit)

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

## Note: gitpod Error(under survey)

```
{
sudo -E /app/bob build
}; exit
bash-5.1$ {
> sudo -E /app/bob build
> }; exit
{"level":"debug","message":"buildkitd started","serviceContext":{"service":"bob","version":""},"severity":"DEBUG","stderr":"/tmp/buildkitd_stderr2907019644","stdout":"/tmp/buildkitd_stdout314613672","time":"2022-08-16T11:15:17Z"}
{"attempt":0,"level":"debug","message":"attempting to connect to buildkitd","serviceContext":{"service":"bob","version":""},"severity":"DEBUG","time":"2022-08-16T11:15:17Z"}
{"attempt":1,"level":"debug","message":"attempting to connect to buildkitd","serviceContext":{"service":"bob","version":""},"severity":"DEBUG","time":"2022-08-16T11:15:18Z"}
{"level":"info","message":"building base image","serviceContext":{"service":"bob","version":""},"severity":"INFO","time":"2022-08-16T11:15:18Z"}
{"level":"info","message":"waiting for build context","serviceContext":{"service":"bob","version":""},"severity":"INFO","time":"2022-08-16T11:15:18Z"}
#1 [internal] load .dockerignore
#1 transferring context: 2B done
#1 DONE 0.0s
...
#5 [2/3] RUN bash -cl "rustup toolchain install stable && rustup target add wasm32-unknown-unknown"
#5 0.382 info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
#5 0.628 info: latest update on 2022-08-11, rust version 1.63.0 (4b91a6ea7 2022-08-08)
#5 0.668 info: downloading component 'cargo'
#5 0.813 info: downloading component 'clippy'
#5 0.889 info: downloading component 'rls'
#5 1.011 info: downloading component 'rust-analysis'
#5 1.382 info: downloading component 'rust-src'
#5 1.490 info: downloading component 'rust-std'
#5 1.935 info: downloading component 'rustc'
#5 3.085 info: downloading component 'rustfmt'
#5 3.204 info: removing previous version of component 'cargo'
#5 3.340 info: rolling back changes
#5 3.345 error: could not rename component file from '/home/gitpod/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/zsh/site-functions' to '/home/gitpod/.rustup/tmp/59ys9kk5jjoqg1jc_dir/bk': Invalid cross-device link (os error 18)
#5 ERROR: process "/bin/sh -c bash -cl \"rustup toolchain install stable && rustup target add wasm32-unknown-unknown\"" did not complete successfully: exit code: 1
------
 > [2/3] RUN bash -cl "rustup toolchain install stable && rustup target add wasm32-unknown-unknown":
#5 0.813 info: downloading component 'clippy'
#5 0.889 info: downloading component 'rls'
#5 1.011 info: downloading component 'rust-analysis'
#5 1.382 info: downloading component 'rust-src'
#5 1.490 info: downloading component 'rust-std'
#5 1.935 info: downloading component 'rustc'
#5 3.085 info: downloading component 'rustfmt'
#5 3.204 info: removing previous version of component 'cargo'
#5 3.340 info: rolling back changes
#5 3.345 error: could not rename component file from '/home/gitpod/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/zsh/site-functions' to '/home/gitpod/.rustup/tmp/59ys9kk5jjoqg1jc_dir/bk': Invalid cross-device link (os error 18)
------
.gitpod.Dockerfile:3
--------------------
   1 |     FROM gitpod/workspace-full
   2 |     
   3 | >>> RUN bash -cl "rustup toolchain install stable && rustup target add wasm32-unknown-unknown"
   4 |     
   5 |     RUN bash -c ". .nvm/nvm.sh \
--------------------
error: failed to solve: process "/bin/sh -c bash -cl \"rustup toolchain install stable && rustup target add wasm32-unknown-unknown\"" did not complete successfully: exit code: 1
{"@type":"type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent","command":"build","error":"exit status 1","level":"error","message":"build failed","serviceContext":{"service":"bob","version":""},"severity":"ERROR","time":"2022-08-16T11:17:58Z"}
exit
```
