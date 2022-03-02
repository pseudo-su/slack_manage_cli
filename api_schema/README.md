# Slack API specs

The Slack OpenAPI are taken from the [here](https://github.com/pseudo-su/slack-specs).

## Generating the rust client

A rust client is generated from the OpenAPI specs following similar steps as described [here](https://www.twilio.com/docs/openapi/generating-a-rust-client-for-twilios-api#setup)

```sh
# install the client generator
brew install openapi-generator

# Generate client into temp directory
openapi-generator generate -g rust \
  -i complete.openapi.yaml \
  -o ./tmp

# copy useful bit out of the generated client
cp -r ./tmp/src/ ./src/api_client/

mv ./src/api_client/lib.rs ./src/api_client/mod.rs
```

You may still need to:

- Manually update `use crate::*` references
- Remove/hoist parts of `mod.rs` into the crate root
