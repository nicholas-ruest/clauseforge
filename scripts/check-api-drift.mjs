#!/usr/bin/env node
import {readFileSync} from "node:fs";
import {generateClient} from "./generate-api-client.mjs";
const schema=JSON.parse(readFileSync(new URL("../api/openapi.json",import.meta.url)));
const client=readFileSync(new URL("../frontend/src/api/client.ts",import.meta.url),"utf8");
const generated=readFileSync(new URL("../frontend/src/api/generated.ts",import.meta.url),"utf8");
const expected=generateClient(schema);
if(generated!==expected)throw new Error("generated TypeScript contract differs structurally from api/openapi.json; run node scripts/generate-api-client.mjs");
if(!client.includes('from "./generated"'))throw new Error("runtime client does not import the generated contract");
for(const operation of Object.values(schema.paths).flatMap(path=>Object.values(path)).map(value=>value.operationId)){
  if(!generated.includes(`${operation}:`))throw new Error(`generated client omits operation ${operation}`);
}
console.log(`API_DRIFT_PASS schemas=${Object.keys(schema.components.schemas).length} operations=${Object.values(schema.paths).flatMap(path=>Object.values(path)).length}`);
