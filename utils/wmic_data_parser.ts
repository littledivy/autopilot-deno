// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

function parse(yml: Promise<string>) {
  return new Promise((resolve, reject) => {
    yml.then(x => {
      let arr = x.split("\n");
      let obj: any[] = [];
      arr.forEach(element => {
        obj.push(element.split(":")[1])
      });
      resolve(obj);
    });
  });
}

export default function parseMonitorsWin(info: Promise<string>): Promise<string> {
  return new Promise((resolve, reject) => {
  parse(info).then(result => {
    let displayLength = 0;
    // @ts-ignore
    for (var i in result) {
      // @ts-ignore
      if (result[i]) {
      // @ts-ignore
      if(result[i].trim() == "") continue;
      else displayLength++;
      }
    }
      resolve(makeParseableString(displayLength));
  });
});
}

function makeParseableString(len: number) {
  return `Monitors: ${len}`;
}
