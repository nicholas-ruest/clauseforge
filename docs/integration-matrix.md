# Integration matrix

| Project and revision | Product responsibility | Rust call site | Success marker | Failure proof |
|---|---|---|---|---|
| RuVector `979c31a` | retrieve comparable obligation evidence | upstream-ruvector | `similarity_set` | corrupt index load |
| ruv-FANN `02431fd` | calibrate amendment exposure | upstream-fann | `calibrated_interval` | invalid quantile/config |
| midstream `0934fd3` | bound control-signal windows | upstream-midstream | `safety_window` | constraint violation |
| QuDAG `b534668` | protect reviewer evidence envelope | upstream-qudag | `kem_ciphertext` | security-level mismatch |
| dspy.ts `200c03a` | rank affected clauses stably | upstream-dspy | `dspy_rank` | invalid limit selects nothing |
| Guardrail `65df339` | validate nested amendment facts | upstream-guardrail | `guardrail_valid` | missing nested field denies |
| OPA `ba39415` | authorize amendment decision | upstream-opa | `policy_allow` | denied policy input |
| LogAct `e24c1fb` | generate deterministic mitigation actions | upstream-logact | `logact_policy` | unsupported policy rejected |

All wrappers verify a 40-character checkout HEAD before execution. Adapter tests additionally prove non-zero exit and missing-marker behavior. Coordination acknowledgments are never included as product integrations.
