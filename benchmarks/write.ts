// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { writeJsonSync } from "https://deno.land/std/fs/mod.ts";

var columns = `|Name|Time(ms)|\n|----|----|`;
var rows = ``;

export default function write(data: any) {
  for (let i = 0; i < data.results.length; i++) {
    rows += `|${data.results[i].name}|${data.results[i].totalMs}|\n`;
  }
  let table = `${columns}\n${rows}`;
  Deno.writeTextFile("benchmarks/README.md", table);
  Deno.writeTextFile("docs/benchmarks.md", table);
  writeJsonSync("benchmarks/benchmarks.dat", data, { spaces: 2 });
}
