// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import {
  createLogger,
  LogLevel,
  consoleSink,
  fileSink,
  jsonFormat,
  textFormat,
} from "https://deno.land/x/deno_structured_logging@0.4.1/mod.ts";

// use autopilots own plugin prepare
export { prepare } from "https://raw.githubusercontent.com/divy-work/autopilot-plugin-prepare/master/mod.ts";

export const logger = createLogger({
  minimumLevel: Deno.env.get("DEBUG") ? LogLevel.DEBUG : LogLevel.INFO,
  outputFormat: textFormat,
}).addSink(consoleSink());
