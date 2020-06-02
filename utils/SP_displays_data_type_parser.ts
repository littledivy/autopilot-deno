export default function parseMonitorsMac(info: string): string {
  const result = parse(info);
  let displayList;
  for (var i in result["Graphics/Displays"]) {
    for (var j in result["Graphics/Displays"][i]) {
      if (j == "Displays") {
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
  let length:number = 0;
  for (var x in displayList) {
    length++;
  }
  return length;
}
