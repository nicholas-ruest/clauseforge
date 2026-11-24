import { expect, test } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
const root=fileURLToPath(new URL("../..",import.meta.url));

test("[journey:primary] reviewer completes a source-bound amendment",async({page})=>{
  await page.goto("/");
  await expect(page.getByRole("heading",{name:/trace the clause change/i})).toBeVisible();
  await page.getByLabel("AI supplier").fill("VectorCloud AI");
  await page.getByLabel("Proposed amendment").fill("Permit cross-region model telemetry with a 24-hour incident notice");
  await page.getByRole("button",{name:/analyze amendment impact/i}).click();
  await expect(page.getByText("Impact review completed")).toBeVisible({timeout:120_000});
  await expect(page.getByTestId("result")).toContainText("18 accountable stages");
  const receipts=page.getByRole("article").filter({has:page.getByText(/upstream-/)});
  await expect(page.getByLabel("Upstream evidence receipts")).toContainText("upstream-ruvector");
  await expect(page.getByLabel("Upstream evidence receipts")).toContainText("similarity_set");
  await expect(receipts.first()).toBeVisible();
  await page.screenshot({path:"../.evidence/screenshots/primary-desktop.png",fullPage:true});
});

test("[journey:denial-recovery] policy denial writes nothing and recovery succeeds",async({page,request})=>{
  const before=(await (await request.get("http://127.0.0.1:8787/api/v1/audit")).json() as unknown[]).length;
  await page.goto("/");
  await page.getByLabel("Reviewer role").selectOption("observer");
  await page.getByRole("button",{name:/analyze amendment impact/i}).click();
  await expect(page.getByRole("alert")).toContainText("policy denied");
  const after=(await (await request.get("http://127.0.0.1:8787/api/v1/audit")).json() as unknown[]).length;
  expect(after).toBe(before);
  await page.getByRole("button",{name:/restore reviewer access/i}).click();
  await page.getByRole("button",{name:/analyze amendment impact/i}).click();
  await expect(page.getByText("Impact review completed")).toBeVisible({timeout:120_000});
});

test("[journey:durable-retrieval] refresh renders saved hash-chain evidence",async({page})=>{
  execFileSync(`${root}/scripts/stop.sh`,{stdio:"inherit"});
  execFileSync(`${root}/scripts/start.sh`,{stdio:"inherit"});
  await page.goto("/");
  await page.getByRole("button",{name:/audit ledger/i}).click();
  await expect(page.getByTestId("evidence-card").first()).toBeVisible();
  await expect(page.getByTestId("evidence-card").first()).toContainText("Upstream receipts");
  await expect(page.getByTestId("evidence-card").first()).toContainText("18");
  const correlation=await page.getByTestId("evidence-card").first().locator("dd").first().textContent();
  await page.reload();
  await page.getByRole("button",{name:/audit ledger/i}).click();
  await expect(page.getByTestId("evidence-card").first()).toContainText(correlation ?? "missing");
  await page.setViewportSize({width:390,height:844});
  await page.screenshot({path:"../.evidence/screenshots/durable-mobile.png",fullPage:true});
});
