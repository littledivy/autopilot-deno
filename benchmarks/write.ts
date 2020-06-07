// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { writeJsonSync } from "https://deno.land/std/fs/mod.ts";

export default function write(data: object) {
  writeJsonSync("benchmarks/benchmarks.dat", data, { spaces: 2 });
}
