import { expect, test } from '@playwright/test';

const runtimeFailures = new WeakMap();

test.beforeEach(async ({ page }) => {
  const failures = [];
  runtimeFailures.set(page, failures);
  page.on('console', (message) => {
    if (message.type() === 'error') failures.push(`console: ${message.text()}`);
  });
  page.on('pageerror', (error) => failures.push(`page: ${error.message}`));
  page.on('request', (request) => {
    const url = new URL(request.url());
    if (url.hostname !== '127.0.0.1') failures.push(`external request: ${url}`);
  });

  await page.goto('/static/');
  await expect(page.getByRole('status', { name: 'Conversion engine status' })).toHaveText('Ready');
});

test.afterEach(async ({ page }) => {
  expect(runtimeFailures.get(page)).toEqual([]);
});

test('initializes real WASM without horizontal page overflow', async ({ page }) => {
  const dimensions = await page.evaluate(() => ({
    clientWidth: document.documentElement.clientWidth,
    scrollWidth: document.documentElement.scrollWidth,
  }));
  expect(dimensions.scrollWidth).toBeLessThanOrEqual(dimensions.clientWidth);
  await expect(page.getByRole('heading', { name: 'JSON YAML Swiss' })).toBeVisible();
});

test('reports ambiguous format detection without changing the source selection', async ({ page }) => {
  await page.getByLabel('Source format').selectOption('yaml');
  await page.getByLabel('Configuration input').fill('true');
  await page.getByRole('button', { name: 'Detect format' }).click();

  await expect(page.getByTestId('detection-result')).toContainText('JSON');
  await expect(page.getByTestId('detection-result')).toContainText('YAML');
  await expect(page.getByTestId('detection-result')).toContainText('Ambiguous');
  await expect(page.getByLabel('Source format')).toHaveValue('yaml');
});

test('converts with an explicit source and exposes normalization warnings', async ({ page }) => {
  await page.getByLabel('Source format').selectOption('json');
  await page.getByLabel('Target format').selectOption('yaml');
  await page.getByLabel('Configuration input').fill('{"z":1,"agent":{"enabled":true}}');
  await page.getByRole('button', { name: 'Convert configuration' }).click();

  await expect(page.getByLabel('Conversion output')).toHaveValue(/agent:/);
  await expect(page.getByTestId('root-type')).toHaveText('Object');
  await expect(page.getByTestId('warning-list')).toContainText('KEY_ORDER_NORMALIZED');
});

test('shows stable errors for invalid and unrepresentable input', async ({ page }) => {
  await page.getByLabel('Source format').selectOption('json');
  await page.getByLabel('Configuration input').fill('{broken');
  await page.getByRole('button', { name: 'Inspect input' }).click();
  await expect(page.getByRole('alert')).toContainText('INVALID_JSON');

  await page.getByLabel('Configuration input').fill('{"missing":null}');
  await page.getByLabel('Target format').selectOption('toml');
  await page.getByRole('button', { name: 'Convert configuration' }).click();
  await expect(page.getByRole('alert')).toContainText('TARGET_CANNOT_REPRESENT_VALUE');
});

test('renders adversarial configuration as text', async ({ page }) => {
  const payload = '{"markup":"<img src=x onerror=window.__executedMarkup=true>"}';
  await page.getByLabel('Source format').selectOption('json');
  await page.getByLabel('Target format').selectOption('yaml');
  await page.getByLabel('Configuration input').fill(payload);
  await page.getByRole('button', { name: 'Convert configuration' }).click();

  await expect(page.getByLabel('Conversion output')).toHaveValue(/<img src=x onerror=window.__executedMarkup=true>/);
  expect(await page.evaluate(() => window.__executedMarkup)).toBeUndefined();
});

test('loads UTF-8 files and rejects unsafe file boundaries', async ({ page }) => {
  await page.getByLabel('Open configuration file').setInputFiles({
    name: 'agent.yaml',
    mimeType: 'application/yaml',
    buffer: Buffer.from('agent:\n  enabled: true\n'),
  });
  await expect(page.getByLabel('Configuration input')).toHaveValue('agent:\n  enabled: true\n');

  await page.getByLabel('Open configuration file').setInputFiles({
    name: 'invalid.yaml',
    mimeType: 'application/yaml',
    buffer: Buffer.from([0xc3, 0x28]),
  });
  await expect(page.getByRole('alert')).toContainText('INVALID_UTF8');

  await page.getByLabel('Open configuration file').setInputFiles({
    name: 'large.json',
    mimeType: 'application/json',
    buffer: Buffer.alloc((2 * 1024 * 1024) + 1, 32),
  });
  await expect(page.getByRole('alert')).toContainText('INPUT_TOO_LARGE');
});

test('offers visible keyboard focus and a Chinese interface', async ({ page }) => {
  await page.keyboard.press('Tab');
  const focus = await page.evaluate(() => {
    const style = getComputedStyle(document.activeElement);
    return { text: document.activeElement.textContent?.trim(), width: style.outlineWidth };
  });
  expect(focus.text).toBe('Skip to workbench');
  expect(Number.parseFloat(focus.width)).toBeGreaterThanOrEqual(2);

  await page.getByRole('button', { name: 'Switch to Chinese' }).click();
  await expect(page.getByRole('heading', { name: 'JSON YAML Swiss' })).toBeVisible();
  await expect(page.getByRole('button', { name: '转换配置' })).toBeVisible();
  expect(await page.locator('html').getAttribute('lang')).toBe('zh-CN');
});

test('keeps the file workflow reachable from the keyboard', async ({ page }) => {
  await page.getByLabel('Configuration input').focus();
  await page.keyboard.press('Tab');

  await expect(page.getByRole('button', { name: 'Open file' })).toBeFocused();
});

test('preserves completed states when switching to Chinese', async ({ page }) => {
  await page.getByRole('button', { name: 'Load sample' }).click();
  await page.getByRole('button', { name: 'Convert configuration' }).click();
  await page.getByRole('button', { name: 'Switch to Chinese' }).click();

  await expect(page.locator('#output-state')).toHaveText('转换完成');
  await expect(page.locator('#inspection-state')).toHaveText('已检查');
  await expect(page.getByRole('status', { name: '转换引擎状态' })).toHaveText('就绪');
});

test('copies and downloads the exact converted output', async ({ context, page }) => {
  await context.grantPermissions(['clipboard-read', 'clipboard-write']);
  await page.getByRole('button', { name: 'Load sample' }).click();
  await page.getByRole('button', { name: 'Convert configuration' }).click();
  const output = await page.getByLabel('Conversion output').inputValue();

  await page.getByRole('button', { name: 'Copy' }).click();
  expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(output);

  const downloadPromise = page.waitForEvent('download');
  await page.getByRole('button', { name: 'Download' }).click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe('converted.yaml');
  const stream = await download.createReadStream();
  const chunks = [];
  for await (const chunk of stream) chunks.push(chunk);
  expect(Buffer.concat(chunks).toString('utf8')).toBe(output);
});
