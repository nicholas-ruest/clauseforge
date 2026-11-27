import { useEffect, useMemo, useState } from "react";
import type { FormEvent } from "react";
import { api, type EvidenceRecord, type AmendmentResponse } from "./api/client";
import "./evidence.css";

type View = "studio" | "evidence";
const defaults = {supplier:"VectorCloud AI",amendment:"Permit cross-region model telemetry and reduce breach notice from 72 to 24 hours",affected_controls:35,cure_days:12};

function EvidenceCard({record}:{record:EvidenceRecord}) {
  const payload = useMemo(()=>{try{return JSON.parse(record.payload) as {supplier?:string;amendment?:string;risk_score?:number;stage_evidence?:Array<{stage:string;score:number;evidence:string[]}>}}catch{return {}}},[record.payload]);
  return <article className="evidence-card" data-testid="evidence-card">
    <div><span className="status-dot"/>sealed</div><h3>{payload.supplier ?? "Amendment evidence"}</h3>
    <p>{payload.amendment ?? "Recorded operational amendment"}</p>
    <dl><div><dt>Correlation</dt><dd>{record.correlation_id}</dd></div><div><dt>Risk</dt><dd>{typeof payload.risk_score==="number"?payload.risk_score.toFixed(2):"n/a"}</dd></div><div><dt>Upstream receipts</dt><dd>{payload.stage_evidence?.length ?? 0}</dd></div></dl>
    <code>{record.hash.slice(0,22)}...</code>
  </article>;
}

export function App() {
  const [view,setView]=useState<View>("studio");
  const [form,setForm]=useState(defaults);
  const [role,setRole]=useState("amendment-reviewer");
  const [result,setResult]=useState<AmendmentResponse|null>(null);
  const [records,setRecords]=useState<EvidenceRecord[]>([]);
  const [loading,setLoading]=useState(false);
  const [auditLoading,setAuditLoading]=useState(true);
  const [error,setError]=useState("");

  async function refreshAudit(){setAuditLoading(true);try{setRecords(await api.audit())}catch(e){setError(e instanceof Error?e.message:"Audit unavailable")}finally{setAuditLoading(false)}}
  useEffect(()=>{void refreshAudit()},[]);
  async function submit(event:FormEvent){event.preventDefault();setLoading(true);setError("");setResult(null);try{const created=await api.create(form,role);setResult(created);await refreshAudit()}catch(e){setError(e instanceof Error?e.message:"Amendment failed")}finally{setLoading(false)}}

  return <div className="app-shell">
    <header><a className="brand" href="#main" aria-label="ClauseForge home"><span className="brand-mark">CF</span><span>CLAUSEFORGE<small>AMENDMENT IMPACT STUDIO</small></span></a>
      <nav aria-label="Primary"><button className={view==="studio"?"active":""} onClick={()=>setView("studio")}>Impact studio</button><button className={view==="evidence"?"active":""} onClick={()=>setView("evidence")} >Audit ledger <b>{records.length}</b></button></nav>
      <div className="live"><span/>local / ready</div></header>
    <main id="main">
      {view==="studio"?<>
        <section className="hero"><div><p className="eyebrow">SUPPLIER AI GOVERNANCE</p><h1>Trace the clause change.<br/><em>Protect every control.</em></h1><p>Map a supplier AI amendment to obligations, control owners, policy boundaries, and an auditable mitigation plan before approval.</p></div><div className="mesh" aria-hidden="true"><i/><i/><i/><i/></div></section>
        <section className="studio-grid">
          <form onSubmit={submit} aria-label="Amendment composer"><div className="panel-title"><span>01</span><div><h2>Compose amendment</h2><p>Define the contractual change and the control surface it can affect.</p></div></div>
            <label>AI supplier<input value={form.supplier} onChange={e=>setForm({...form,supplier:e.target.value})} required/></label>
            <label>Proposed amendment<textarea value={form.amendment} onChange={e=>setForm({...form,amendment:e.target.value})} required/></label>
            <div className="row"><label>Affected controls <output>{form.affected_controls}%</output><input aria-label="Affected controls" type="range" min="1" max="100" value={form.affected_controls} onChange={e=>setForm({...form,affected_controls:Number(e.target.value)})}/></label><label>Contractual cure period<input aria-label="Cure days" type="number" min="1" max="365" value={form.cure_days} onChange={e=>setForm({...form,cure_days:Number(e.target.value)})}/><span className="unit">DAYS</span></label></div>
            <label>Reviewer role<select value={role} onChange={e=>setRole(e.target.value)}><option value="amendment-reviewer">Amendment reviewer</option><option value="observer">Observer (denied)</option></select></label>
            <button className="launch" disabled={loading}>{loading?<><span className="spinner"/>Running eight integrations...</>:"Analyze amendment impact"}</button>
          </form>
          <section className="result-panel" aria-live="polite"><div className="panel-title"><span>02</span><div><h2>Decision evidence</h2><p>Correlated results from the execution path.</p></div></div>
            {error&&<div className="error" role="alert"><strong>Boundary held</strong><p>{error}</p><button onClick={()=>{setRole("amendment-reviewer");setError("")}}>Restore reviewer access</button></div>}
            {!error&&!result&&!loading&&<div className="empty"><div className="radar"/><h3>Awaiting amendment</h3><p>Results, upstream receipts, and the evidence seal will appear here.</p></div>}
            {loading&&<div className="empty"><div className="radar running"/><h3>Executing complete path</h3><p>Policy, domain stages, and upstream responsibilities are being correlated.</p></div>}
            {result&&<div className="success" data-testid="result"><div className="score"><span>Residual amendment risk</span><strong>{result.risk_score.toFixed(2)}</strong></div><h3>Impact review completed</h3><p>{result.stages.length} accountable stages returned evidence.</p><div className="stage-list">{result.stages.map(stage=><span key={stage}>{stage}</span>)}</div><div className="upstream-receipts" aria-label="Upstream evidence receipts">{result.stage_evidence.map(receipt=><article key={receipt.stage}><h4>{receipt.stage}</h4><strong>{receipt.score.toFixed(2)}</strong><code>{receipt.evidence[0] ?? "verified gate"}</code></article>)}</div><dl><div><dt>Correlation</dt><dd>{result.correlation_id}</dd></div><div><dt >Audit ledger seal</dt><dd>{result.evidence_hash.slice(0,26)}...</dd></div></dl></div>}
          </section>
        </section></>:<section className="vault"><p className="eyebrow">CONTRACT DECISION LEDGER</p><h1>Every amendment decision keeps its sources, controls, and seal.</h1><button onClick={()=>void refreshAudit()}>Refresh audit</button>{auditLoading?<p className="loading-copy">Loading sealed records...</p>:records.length===0?<div className="empty"><h3>No evidence yet</h3><p>Complete an amendment to create the first hash-chained record.</p></div>:<div className="evidence-grid">{records.map(r=><EvidenceCard key={r.id} record={r}/>)}</div>}</section>}
    </main>
    <footer><span>ClauseForge / local-first</span><span>8 upstreams · 26 Rust crates · immutable audit</span></footer>
  </div>;
}
