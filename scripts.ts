// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.
// Scripts for velociraptor

export default {
  "scripts": {
    "build": "cargo build",
    "bench": "DEV=* DEBUG=* deno run --unstable -A benchmarks/index.ts",
    "fmt": "deno fmt && cargo fmt",
    "test": "deno test --unstable -A",
  },
};
