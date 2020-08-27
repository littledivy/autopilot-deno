// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

export default function filename(filenameBase: string): string {
  let filenameSuffix = ".so";
  let filenamePrefix = "lib";

  if (Deno.build.os === "windows") {
    filenameSuffix = ".dll";
    filenamePrefix = "";
  }
  if (Deno.build.os === "darwin") {
    filenameSuffix = ".dylib";
  }

  return `${filenamePrefix}${filenameBase}${filenameSuffix}`;
}
