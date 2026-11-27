#!/usr/bin/env node
import {readFileSync,writeFileSync} from "node:fs";
import {fileURLToPath} from "node:url";

function tsType(schema){
  if(schema.$ref)return schema.$ref.split("/").at(-1);
  if(schema.type==="string")return "string";
  if(schema.type==="integer"||schema.type==="number")return "number";
  if(schema.type==="boolean")return "boolean";
  if(schema.type==="array")return `${tsType(schema.items)}[]`;
  throw new Error(`unsupported OpenAPI type ${schema.type}`);
}

export function generateClient(schema){
  const lines=[
    "// Generated from api/openapi.json by scripts/generate-api-client.mjs.",
    "// Do not edit by hand.",
    "",
  ];
  for(const [name,definition] of Object.entries(schema.components.schemas)){
    if(definition.type!=="object")throw new Error(`schema ${name} must be an object`);
    const required=new Set(definition.required??[]);
    const fields=Object.entries(definition.properties).map(([field,value])=>
      `  ${field}${required.has(field)?"":"?"}: ${tsType(value)};`
    );
    lines.push(`export type ${name} = {`,...fields,"};","");
  }
  lines.push("export const API_OPERATIONS = {");
  for(const [path,item] of Object.entries(schema.paths)){
    for(const [method,operation] of Object.entries(item)){
      if(!operation.operationId)throw new Error(`${method.toUpperCase()} ${path} has no operationId`);
      lines.push(`  ${operation.operationId}: { method: "${method.toUpperCase()}", path: "${path}" },`);
    }
  }
  lines.push("} as const;");
  return `${lines.join("\n")}\n`;
}

const invoked=process.argv[1]&&fileURLToPath(import.meta.url)===process.argv[1];
if(invoked){
  const schemaUrl=new URL("../api/openapi.json",import.meta.url);
  const outputUrl=new URL("../frontend/src/api/generated.ts",import.meta.url);
  const schema=JSON.parse(readFileSync(schemaUrl,"utf8"));
  writeFileSync(outputUrl,generateClient(schema));
  console.log(`generated ${fileURLToPath(outputUrl)}`);
}
