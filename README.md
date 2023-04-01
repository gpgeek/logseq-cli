# logseq-cli
This is really just an educational toy project at this time.
I'm releasing this because someone on reddit had the same use-case and asked nicely :)
## Build a binary
1. install rust
2. run `cargo build --release`
3. binary is located here  `./target/release/`

## Run Logseq API
1. https://docs.logseq.com/#/page/local%20http%20server

## Set Envvars
1. `export LOGSEQ_API_KEY=<YOUR-KEY>`
2. `export LOGSEQ_API_URL=<YOUR-SERVER_URL>`
    default is "http://127.0.0.1:12315/api"

## Run it
  `logseq-cli journal "my journal note"`
  `logseq-cli j "my journal note"`
  This literally the only command available ATM.