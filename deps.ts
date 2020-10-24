// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { Houston, ConsoleTransport } from "https://deno.land/x/houston/mod.ts";
export const logger = new Houston([new ConsoleTransport()]);

export { prepare } from "https://deno.land/x/plugin_prepare@v0.8.0/mod.ts";
