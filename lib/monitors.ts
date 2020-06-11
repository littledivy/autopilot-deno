// Copyright 2020-present Divy Srivastava and friends. All rights reserved. MIT license.

// THIS IS A FALLBACK -- CURRENTLY USED FOR WINDOWS
// monitors.ts (this file was an idea that has been implemented in windows.rs)

const WinArgs = [
  "cmd",
  "/C",
  `for /F %%M in (' wmic path Win32_PnPEntity where "Service='monitor' and Status='OK'" get DeviceID /VALUE ') do echo Monitors: %%M`,
];

async function runCommand(cliArgs: string[]): Promise<string> {
  const process = Deno.run({
    cmd: [...cliArgs],
    cwd: "utils",
    stdout: "piped",
  });
  const { code } = await process.status();
  if (code === 0) {
    const rawOutput = await process.output();
    process.close();
    return new TextDecoder().decode(rawOutput);
  } else {
    process.close();
    throw new Error(`Process exited with error code ${code}`);
  }
}

export async function getMonitors() {
  switch (Deno.build.os) {
    case "windows":
      const res = await runCommand(WinArgs);
      return res.split("\n")[0].split("Monitors:").join("").trim();
      break;
    default:
      return "1"
      break;
  }
}
