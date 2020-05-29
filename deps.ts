import {
  createLogger,
  LogLevel,
  consoleSink,
  fileSink,
  jsonFormat,
  textFormat,
} from "https://deno.land/x/deno_structured_logging@0.4.1/mod.ts";

export const logger = createLogger({
  minimumLevel: Deno.env.get("DEBUG") ? LogLevel.DEBUG : LogLevel.INFO,
  outputFormat: textFormat,
}).addSink(consoleSink());
