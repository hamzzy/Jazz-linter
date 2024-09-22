import init, { Linter } from '../pkg/Jazz_linter.js';

async function runLinter(code) {
  await init();
  const linter = new Linter();
  const result = linter.lint(code);
  return JSON.parse(result);
}

export { runLinter };