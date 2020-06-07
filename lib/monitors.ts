// monitors.ts (this file was an idea that has been implemented in windows.rs)

const WinArgs = [
  "cmd",
  "/C",
  "monitors.bat",
];

const MacArgs = [
  "system_profiler",
  "SPDisplaysDataType",
];

const LinuxArgs = [
  "xrandr",
  "--listactivemonitors",
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
    case "linux":
      const res = await runCommand(LinuxArgs);
      return res.split("\n")[0].split("Monitors:").join("").trim();
      break;
  }
}
