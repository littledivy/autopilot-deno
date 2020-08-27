// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { Houston, ConsoleTransport } from "https://deno.land/x/houston/mod.ts";
export const logger = new Houston([new ConsoleTransport()]);

export { Plug as prepare } from "https://x.nest.land/plug@0.0.5/mod.ts";
