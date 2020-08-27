export function isAscii(str: string): boolean {
  return /^[\x00-\x7F]*$/.test(str);
}

export function throwAsciiError() {
  throw new TypeError("String is not a valid ascii.");
}
