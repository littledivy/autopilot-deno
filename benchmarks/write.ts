import { writeJsonSync } from "https://deno.land/std/fs/mod.ts";

export default function write(data: object) {
  writeJsonSync("benchmarks/benchmarks.dat", data, { spaces: 2 });
}
