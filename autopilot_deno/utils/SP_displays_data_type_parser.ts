// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

import { parse } from "https://deno.land/std/encoding/yaml.ts";

export default function parseMonitorsMac(info: string): string {
  const result = parse(info);
  let displayList;
  // @ts-ignore
  for (var i in result["Graphics/Displays"]) {
    // @ts-ignore
    for (var j in result["Graphics/Displays"][i]) {
      if (j == "Displays") {
        // @ts-ignore
        displayList = result["Graphics/Displays"][i][j];
        break;
      }
    }
  }
  return makeParseableString(parseDisplayListLength(displayList));
}

function makeParseableString(len: number) {
  return `Monitors: ${len}`;
}

function parseDisplayListLength(displayList: object): number {
  let length: number = 0;
  for (var x in displayList) {
    length++;
  }
  return length;
}
