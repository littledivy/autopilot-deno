async function downloadFromRemote(
  name
  remoteUrl
  savePath
) {
  log.info(`downloading deno plugin "${name}" from "${remoteUrl}"`);
  const download = await fetch(remoteUrl);

  if (download.status !== 200) {
    throw Error(`downloading plugin "${name}" from "${remoteUrl}" failed.`);
  }

  const pluginFileData = await download.arrayBuffer();
  await Deno.writeFile(savePath, new Uint8Array(pluginFileData));
}
