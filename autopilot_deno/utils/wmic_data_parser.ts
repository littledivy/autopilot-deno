// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

function parse(yml: string) {
      let arr = yml.split("\n");
      let obj: any[] = [];
      arr.forEach((element) => {
        obj.push(element.split(":")[1]);
      });
      return obj;
}

export default function parseMonitorsWin(
  info: string,
): string {
      let result = parse(info);
      let displayLength = 0;
      // @ts-ignore
      for (var i in result) {
        // @ts-ignore
        if (result[i]) {
          // @ts-ignore
          if (result[i].trim() == "") continue;
          else displayLength++;
        }
      }
      return makeParseableString(displayLength);
}

function makeParseableString(len: number) {
  return `Monitors: ${len}`;
}
