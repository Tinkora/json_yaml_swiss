import assert from 'node:assert/strict';
import { mkdir, mkdtemp, readFile, rm, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

const script = fileURLToPath(new URL('./prepare_wasm_smoke.mjs', import.meta.url));

async function createPackage(root) {
  await mkdir(root, { recursive: true });
  await writeFile(join(root, 'package.json'), '{}\n');
  await writeFile(join(root, 'json_yaml_swiss_web.js'), 'export default async function init() {}\n');
  await writeFile(join(root, 'json_yaml_swiss_web_bg.wasm'), 'wasm');
}

async function fixture(t) {
  const root = await mkdtemp(join(tmpdir(), 'json_yaml_swiss_wasm_'));
  await mkdir(join(root, 'static'), { recursive: true });
  t.after(() => rm(root, { recursive: true, force: true }));
  return root;
}

function run(root, artifact) {
  const env = { ...process.env };
  if (artifact) env.WASM_SMOKE_PACKAGE = artifact;
  else delete env.WASM_SMOKE_PACKAGE;
  return spawnSync(process.execPath, [script], {
    cwd: root,
    encoding: 'utf8',
    env,
  });
}

test('installs the required package files', async (t) => {
  const root = await fixture(t);
  const artifact = join(root, 'artifact');
  await createPackage(artifact);

  const result = run(root, artifact);

  assert.equal(result.status, 0, result.stderr);
  assert.equal(await readFile(join(root, 'static/pkg/json_yaml_swiss_web_bg.wasm'), 'utf8'), 'wasm');
});

test('rejects a package missing the WASM module', async (t) => {
  const root = await fixture(t);
  const artifact = join(root, 'artifact');
  await createPackage(artifact);
  await rm(join(artifact, 'json_yaml_swiss_web_bg.wasm'));

  const result = run(root, artifact);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /missing json_yaml_swiss_web_bg\.wasm/);
});

test('rejects symbolic package files', async (t) => {
  const root = await fixture(t);
  const artifact = join(root, 'artifact');
  await createPackage(artifact);
  await rm(join(artifact, 'json_yaml_swiss_web.js'));
  await writeFile(join(root, 'outside.js'), 'export default 1;\n');
  await symlink(join(root, 'outside.js'), join(artifact, 'json_yaml_swiss_web.js'));

  const result = run(root, artifact);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /missing json_yaml_swiss_web\.js/);
});

test('validates the existing local package when no artifact is provided', async (t) => {
  const root = await fixture(t);
  await createPackage(join(root, 'static/pkg'));

  const result = run(root);

  assert.equal(result.status, 0, result.stderr);
  assert.equal(await readFile(join(root, 'static/pkg/json_yaml_swiss_web_bg.wasm'), 'utf8'), 'wasm');
});

test('rejects an artifact path that overlaps the destination', async (t) => {
  const root = await fixture(t);
  const destination = join(root, 'static/pkg');
  await createPackage(destination);

  const result = run(root, destination);

  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /must not overlap static\/pkg/);
  assert.equal(await readFile(join(destination, 'json_yaml_swiss_web_bg.wasm'), 'utf8'), 'wasm');
});
