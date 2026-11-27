import { pathToFileURL } from "node:url";
import path from "node:path";
const root = process.env.CLAUSEFORGE_UPSTREAM_ROOT;
const { topK } = await import(pathToFileURL(path.join(root, "dspy.ts/src/utils/top-k.ts")));
const supplier = process.env.CLAUSEFORGE_SUPPLIER;
const amendment = process.env.CLAUSEFORGE_AMENDMENT;
const controls = Number(process.env.CLAUSEFORGE_AFFECTED_CONTROLS);
const cureDays = Number(process.env.CLAUSEFORGE_CURE_DAYS);
const rows = [
  { id: "control-impact", risk: controls * 2 },
  { id: "cure-window", risk: Math.max(0, 90 - cureDays) },
  { id: "text-specificity", risk: supplier.length + amendment.length },
];
const ranked = topK(rows, 2, (a, b) => b.risk - a.risk);
if (ranked.length !== 2 || ranked.some((row) => !Number.isFinite(row.risk))) {
  throw new Error("request ranking failed");
}
console.log("dspy_rank:" + ranked.map((row) => row.id).join(",") + ":" + ranked.map((row) => row.risk).join(","));
