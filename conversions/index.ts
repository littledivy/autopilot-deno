// type conversion for Autopilot's methods
export default function safeConvert(from: string, to: "number" | "object"): null | number | object {
  switch (to) {
    case "number":
      return Number(from);
      break;
    case "object":
      return JSON.parse(from);
      break;
  }
}
