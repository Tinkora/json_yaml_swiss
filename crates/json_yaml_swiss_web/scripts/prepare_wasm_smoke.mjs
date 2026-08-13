import { cp, lstat, mkdir, realpath, rm } from 'node:fs/promises';
import { join, resolve } from 'node:path';

const artifact = process.env.WASM_SMOKE_PACKAGE;
const destination = resolve('static/pkg');
const required = ['package.json', 'json_yaml_swiss_web.js', 'json_yaml_swiss_web_bg.wasm'];
const source = artifact ? resolve(artifact) : destination;

const canonicalSource = await realpath(source);
const canonicalDestination = join(await realpath(resolve('static')), 'pkg');

if (
  artifact &&
  (canonicalSource === canonicalDestination ||
    canonicalSource.startsWith(`${canonicalDestination}/`) ||
    canonicalDestination.startsWith(`${canonicalSource}/`))
) {
  throw new Error('WASM package source must not overlap static/pkg');
}

const sourceMetadata = await lstat(source);
if (!sourceMetadata.isDirectory() || sourceMetadata.isSymbolicLink()) {
  throw new Error('WASM package must be a real directory');
}

for (const filename of required) {
  try {
    const metadata = await lstat(join(source, filename));
    if (!metadata.isFile() || metadata.isSymbolicLink()) throw new Error();
  } catch {
    throw new Error(`WASM package is missing ${filename}`);
  }
}

if (artifact) {
  await rm(destination, { recursive: true, force: true });
  await mkdir(destination, { recursive: true });
  for (const filename of required) await cp(join(source, filename), join(destination, filename));
}
