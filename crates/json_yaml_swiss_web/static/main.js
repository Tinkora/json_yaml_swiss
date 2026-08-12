import init, { wasm_convert, wasm_detect, wasm_inspect } from './pkg/json_yaml_swiss_web.js';

const MAX_INPUT_BYTES = 2 * 1024 * 1024;
const encoder = new TextEncoder();

const copy = {
  en: {
    skip: 'Skip to workbench', kicker: 'Local configuration workbench',
    introTitle: 'Review the value before changing the format.',
    introCopy: 'Parsing and conversion run in local WebAssembly. Ambiguous formats stay explicit, and every normalization is reported before you copy or download output.',
    controls: 'Conversion controls', workspace: 'Editor workspace', report: 'Inspection and conversion report',
    sourceFormat: 'Source format', targetFormat: 'Target format', indent: 'Indent', detect: 'Detect format', inspect: 'Inspect input', convert: 'Convert configuration',
    stepInput: '01 / Input', inputTitle: 'Source document', inputLabel: 'Configuration input', inputPlaceholder: '{"agent":{"enabled":true}}',
    openFile: 'Open file', openFileLabel: 'Open configuration file', sample: 'Load sample', clear: 'Clear',
    stepOutput: '02 / Output', outputTitle: 'Normalized copy', outputLabel: 'Conversion output', outputPlaceholder: 'Run a conversion to create an explicit copy.',
    copy: 'Copy', download: 'Download', waiting: 'Waiting', ready: 'Ready', unavailable: 'Unavailable',
    stepInspect: '03 / Inspect', inspectionTitle: 'Document shape', notInspected: 'Not inspected', inspected: 'Inspected',
    format: 'Format', root: 'Root', nodes: 'Nodes', depth: 'Depth', bytes: 'Bytes',
    stepDetect: '04 / Detect', detectionTitle: 'Parser matches', detectionEmpty: 'Detection is advisory and never changes your source selection.',
    stepWarnings: '05 / Warnings', warningsTitle: 'Normalization review', warningsEmpty: 'No conversion report yet.', noWarnings: 'No normalization warnings.',
    footerPrivacy: 'No upload, storage, telemetry, or JavaScript parser fallback.', ambiguous: 'Ambiguous', suggestion: 'Suggestion', candidates: 'Matches',
    converted: 'Converted', copied: 'Copied to clipboard', downloaded: 'Download created', sampleLoaded: 'Sample loaded',
    languageAria: 'Switch to Chinese', githubAria: 'Open GitHub repository', swapAria: 'Swap formats', statusAria: 'Conversion engine status',
    invalidUtf8: 'The selected file is not valid UTF-8 text.', inputTooLarge: 'The selected file exceeds the 2 MiB limit.', wasmInit: 'The local conversion engine could not be initialized.',
    error: 'Error', clipboardUnavailable: 'Clipboard access was denied.',
  },
  zh: {
    skip: '跳到配置工作台', kicker: '本地配置工作台', introTitle: '转换格式前，先确认数据含义。',
    introCopy: '解析和转换均在本地 WebAssembly 中完成。格式歧义始终明确展示，复制或下载前可检查全部规范化警告。',
    controls: '转换控制', workspace: '配置编辑区', report: '检查与转换报告',
    sourceFormat: '来源格式', targetFormat: '目标格式', indent: '缩进', detect: '检测格式', inspect: '检查输入', convert: '转换配置',
    stepInput: '01 / 输入', inputTitle: '来源文档', inputLabel: '配置输入', inputPlaceholder: '{"agent":{"enabled":true}}',
    openFile: '打开文件', openFileLabel: '打开配置文件', sample: '载入示例', clear: '清空',
    stepOutput: '02 / 输出', outputTitle: '规范化副本', outputLabel: '转换输出', outputPlaceholder: '执行转换后，这里会生成明确的副本。',
    copy: '复制', download: '下载', waiting: '等待输入', ready: '就绪', unavailable: '不可用',
    stepInspect: '03 / 检查', inspectionTitle: '文档结构', notInspected: '尚未检查', inspected: '已检查',
    format: '格式', root: '根类型', nodes: '节点', depth: '深度', bytes: '字节',
    stepDetect: '04 / 检测', detectionTitle: '解析器匹配', detectionEmpty: '检测仅提供建议，绝不会更改来源格式选择。',
    stepWarnings: '05 / 警告', warningsTitle: '规范化检查', warningsEmpty: '尚无转换报告。', noWarnings: '没有规范化警告。',
    footerPrivacy: '不上传、不存储、无遥测、无 JavaScript parser fallback。', ambiguous: '存在歧义', suggestion: '建议', candidates: '匹配格式',
    converted: '转换完成', copied: '已复制到剪贴板', downloaded: '已创建下载', sampleLoaded: '已载入示例',
    languageAria: '切换到英文', githubAria: '打开 GitHub 仓库', swapAria: '交换格式', statusAria: '转换引擎状态',
    invalidUtf8: '所选文件不是有效的 UTF-8 文本。', inputTooLarge: '所选文件超过 2 MiB 限制。', wasmInit: '本地转换引擎初始化失败。',
    error: '错误', clipboardUnavailable: '剪贴板访问被拒绝。',
  },
};

const elements = {
  runtime: document.querySelector('.runtime'), status: document.querySelector('#wasm-status'), language: document.querySelector('#language-button'), languageLabel: document.querySelector('#language-label'),
  error: document.querySelector('#error-message'), source: document.querySelector('#source-format'), target: document.querySelector('#target-format'), indent: document.querySelector('#indent-size'), swap: document.querySelector('#swap-button'),
  detect: document.querySelector('#detect-button'), inspect: document.querySelector('#inspect-button'), convert: document.querySelector('#convert-button'), input: document.querySelector('#configuration-input'), output: document.querySelector('#conversion-output'),
  openFile: document.querySelector('#open-file-button'), file: document.querySelector('#file-input'), sample: document.querySelector('#sample-button'), clear: document.querySelector('#clear-button'), copy: document.querySelector('#copy-button'), download: document.querySelector('#download-button'),
  inputCount: document.querySelector('#input-count'), outputCount: document.querySelector('#output-count'), outputState: document.querySelector('#output-state'), inspectionState: document.querySelector('#inspection-state'),
  metricFormat: document.querySelector('#metric-format'), rootType: document.querySelector('#root-type'), metricNodes: document.querySelector('#metric-nodes'), metricDepth: document.querySelector('#metric-depth'), metricBytes: document.querySelector('#metric-bytes'),
  detection: document.querySelector('#detection-result'), warnings: document.querySelector('#warning-list'), toast: document.querySelector('#toast'),
};

let wasmReady = false;
let language = 'en';
let toastTimer;

function t(key) { return copy[language][key]; }

function applyLanguage() {
  document.documentElement.lang = language === 'en' ? 'en' : 'zh-CN';
  document.querySelectorAll('[data-i18n]').forEach((node) => { node.textContent = t(node.dataset.i18n); });
  document.querySelectorAll('[data-i18n-placeholder]').forEach((node) => { node.placeholder = t(node.dataset.i18nPlaceholder); });
  document.querySelectorAll('[data-i18n-aria]').forEach((node) => { node.setAttribute('aria-label', t(node.dataset.i18nAria)); });
  elements.status.textContent = wasmReady ? t('ready') : elements.runtime.dataset.state === 'error' ? t('unavailable') : 'Loading';
  elements.languageLabel.textContent = language === 'en' ? '中文' : 'EN';
  elements.language.setAttribute('aria-label', t('languageAria'));
  elements.language.title = t('languageAria');
  elements.status.setAttribute('aria-label', t('statusAria'));
  elements.swap.setAttribute('aria-label', t('swapAria'));
  elements.swap.title = t('swapAria');
  document.querySelector('a[href*="github.com"]').setAttribute('aria-label', t('githubAria'));
}

function byteLength(value) { return encoder.encode(value).byteLength; }
function formatBytes(value) { return value < 1024 ? `${value} B` : `${(value / 1024).toFixed(1)} KiB`; }
function titleCase(value) { return value.charAt(0).toUpperCase() + value.slice(1); }

function setState(element, key, tone) {
  element.dataset.i18n = key;
  element.textContent = t(key);
  if (tone) element.dataset.tone = tone;
  else delete element.dataset.tone;
}

function clearError() {
  elements.error.hidden = true;
  elements.error.replaceChildren();
}

function setError(error) {
  const safeError = error && typeof error === 'object' ? error : {};
  elements.error.replaceChildren();
  const code = document.createElement('strong');
  code.textContent = safeError.code || 'UNKNOWN_ERROR';
  const message = document.createElement('span');
  message.textContent = safeError.message || String(error || 'Unknown error');
  elements.error.append(code, message);
  elements.error.hidden = false;
  setState(elements.outputState, 'error', 'danger');
}

function showToast(message) {
  clearTimeout(toastTimer);
  elements.toast.textContent = message;
  elements.toast.hidden = false;
  toastTimer = setTimeout(() => { elements.toast.hidden = true; }, 1800);
}

function updateActions() {
  const hasInput = elements.input.value.length > 0;
  const hasOutput = elements.output.value.length > 0;
  elements.detect.disabled = !(wasmReady && hasInput);
  elements.inspect.disabled = !(wasmReady && hasInput);
  elements.convert.disabled = !(wasmReady && hasInput);
  elements.clear.disabled = !hasInput && !hasOutput;
  elements.copy.disabled = !hasOutput;
  elements.download.disabled = !hasOutput;
  elements.inputCount.textContent = formatBytes(byteLength(elements.input.value));
  elements.outputCount.textContent = formatBytes(byteLength(elements.output.value));
}

function resetReports() {
  clearError();
  elements.output.value = '';
  setState(elements.outputState, 'waiting');
  setState(elements.inspectionState, 'notInspected');
  for (const metric of [elements.metricFormat, elements.rootType, elements.metricNodes, elements.metricDepth, elements.metricBytes]) metric.textContent = '-';
  elements.detection.className = 'empty-report';
  elements.detection.textContent = t('detectionEmpty');
  elements.warnings.replaceChildren();
  const empty = document.createElement('li');
  empty.className = 'clear-warning';
  empty.textContent = t('warningsEmpty');
  elements.warnings.append(empty);
  updateActions();
}

function renderInspection(report) {
  elements.metricFormat.textContent = report.format.toUpperCase();
  elements.rootType.textContent = titleCase(report.root_type);
  elements.metricNodes.textContent = report.node_count.toLocaleString();
  elements.metricDepth.textContent = report.max_depth.toLocaleString();
  elements.metricBytes.textContent = report.byte_size.toLocaleString();
  setState(elements.inspectionState, 'inspected', 'success');
}

function renderDetection(report) {
  elements.detection.replaceChildren();
  elements.detection.className = 'detection-summary';
  const heading = document.createElement('div');
  heading.className = 'detection-heading';
  for (const format of report.candidates) {
    const tag = document.createElement('span');
    tag.className = 'format-tag';
    tag.textContent = format.toUpperCase();
    heading.append(tag);
  }
  if (report.ambiguous) {
    const ambiguity = document.createElement('span');
    ambiguity.className = 'ambiguity';
    ambiguity.textContent = t('ambiguous');
    heading.append(ambiguity);
  }
  const note = document.createElement('p');
  note.className = 'detection-note';
  note.textContent = `${t('candidates')}: ${report.candidates.map((format) => format.toUpperCase()).join(', ')}${report.suggestion ? ` · ${t('suggestion')}: ${report.suggestion.toUpperCase()}` : ''}`;
  elements.detection.append(heading, note);
}

function renderWarnings(warnings) {
  elements.warnings.replaceChildren();
  if (warnings.length === 0) {
    const item = document.createElement('li');
    item.className = 'clear-warning';
    item.textContent = t('noWarnings');
    elements.warnings.append(item);
    return;
  }
  for (const warning of warnings) {
    const item = document.createElement('li');
    item.textContent = warning;
    elements.warnings.append(item);
  }
}

function runInspection() {
  clearError();
  try {
    renderInspection(wasm_inspect(elements.source.value, elements.input.value));
  } catch (error) {
    setError(error);
  }
}

function runDetection() {
  clearError();
  try {
    renderDetection(wasm_detect(elements.input.value));
  } catch (error) {
    setError(error);
  }
}

function runConversion() {
  clearError();
  try {
    const inspection = wasm_inspect(elements.source.value, elements.input.value);
    const report = wasm_convert(elements.source.value, elements.target.value, elements.input.value, true, Number(elements.indent.value));
    elements.output.value = report.output;
    setState(elements.outputState, 'converted', report.warnings.length > 0 ? 'warning' : 'success');
    renderInspection(inspection);
    renderWarnings(report.warnings);
  } catch (error) {
    elements.output.value = '';
    setError(error);
  }
  updateActions();
}

elements.input.addEventListener('input', resetReports);
elements.source.addEventListener('change', resetReports);
elements.target.addEventListener('change', () => { elements.output.value = ''; renderWarnings([]); updateActions(); });
elements.indent.addEventListener('change', () => { elements.output.value = ''; updateActions(); });
elements.detect.addEventListener('click', runDetection);
elements.inspect.addEventListener('click', runInspection);
elements.convert.addEventListener('click', runConversion);
elements.swap.addEventListener('click', () => {
  const source = elements.source.value;
  elements.source.value = elements.target.value;
  elements.target.value = source;
  if (elements.output.value) elements.input.value = elements.output.value;
  resetReports();
});
elements.sample.addEventListener('click', () => {
  elements.source.value = 'json';
  elements.target.value = 'yaml';
  elements.input.value = '{\n  "agent": {\n    "name": "tinkora",\n    "enabled": true,\n    "tools": ["read", "write"]\n  }\n}';
  resetReports();
  showToast(t('sampleLoaded'));
});
elements.clear.addEventListener('click', () => { elements.input.value = ''; resetReports(); elements.input.focus(); });
elements.openFile.addEventListener('click', () => { elements.file.value = ''; elements.file.click(); });
elements.file.addEventListener('change', async () => {
  const [file] = elements.file.files;
  if (!file) return;
  clearError();
  if (file.size > MAX_INPUT_BYTES) { setError({ code: 'INPUT_TOO_LARGE', message: t('inputTooLarge') }); return; }
  try {
    elements.input.value = new TextDecoder('utf-8', { fatal: true }).decode(await file.arrayBuffer());
    const extension = file.name.split('.').pop()?.toLowerCase();
    if (extension === 'json') elements.source.value = 'json';
    else if (extension === 'yaml' || extension === 'yml') elements.source.value = 'yaml';
    else if (extension === 'toml') elements.source.value = 'toml';
    resetReports();
  } catch {
    setError({ code: 'INVALID_UTF8', message: t('invalidUtf8') });
  }
});
elements.copy.addEventListener('click', async () => {
  try { await navigator.clipboard.writeText(elements.output.value); showToast(t('copied')); }
  catch { setError({ code: 'CLIPBOARD_UNAVAILABLE', message: t('clipboardUnavailable') }); }
});
elements.download.addEventListener('click', () => {
  const blob = new Blob([elements.output.value], { type: 'text/plain;charset=utf-8' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = `converted.${elements.target.value === 'yaml' ? 'yaml' : elements.target.value}`;
  link.click();
  URL.revokeObjectURL(link.href);
  showToast(t('downloaded'));
});
elements.language.addEventListener('click', () => { language = language === 'en' ? 'zh' : 'en'; applyLanguage(); });
document.addEventListener('keydown', (event) => {
  if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') { event.preventDefault(); if (!elements.convert.disabled) runConversion(); }
});

try {
  await init();
  wasmReady = true;
  elements.runtime.dataset.state = 'ready';
  elements.status.textContent = t('ready');
  updateActions();
} catch {
  elements.runtime.dataset.state = 'error';
  elements.status.textContent = t('unavailable');
  setError({ code: 'WASM_INIT_FAILED', message: t('wasmInit') });
}
