import { readFileSync } from "node:fs";

const [baselinePath, currentPath, metric, maxRatio] = process.argv.slice(2);
if (!baselinePath || !currentPath || !metric || !maxRatio) {
  console.error(
    "usage: node compare-metric.mjs <baseline.json> <current.json> <metric> <max_ratio>",
  );
  process.exit(2);
}

const baseline = JSON.parse(readFileSync(baselinePath, "utf8"));
const current = JSON.parse(readFileSync(currentPath, "utf8"));
const b = baseline[metric];
const c = current[metric];

if (typeof b !== "number" || typeof c !== "number") {
  console.error(`Metric ${metric} missing or not numeric.`);
  process.exit(2);
}

const threshold = Number(maxRatio);
if (Number.isNaN(threshold)) {
  console.error(`Invalid max ratio: ${maxRatio}`);
  process.exit(2);
}

let ratio = 0;
let baselineMode = "standard";

if (b === 0) {
  baselineMode = "zero-baseline";
  ratio = c === 0 ? 0 : Number.POSITIVE_INFINITY;
}

if (b !== 0) {
  ratio = (c - b) / b;
}

console.log(
  JSON.stringify(
    { metric, baseline: b, current: c, ratio, baselineMode },
    null,
    2,
  ),
);

if (ratio > threshold) {
  console.error(
    `Regression on ${metric}: ${(ratio * 100).toFixed(2)}% > ${(threshold * 100).toFixed(2)}%`,
  );
  process.exit(1);
}
