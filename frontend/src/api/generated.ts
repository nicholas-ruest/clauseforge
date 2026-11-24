// Generated from api/openapi.json by scripts/generate-api-client.mjs.
// Do not edit by hand.

export type SubmitAmendment = {
  supplier: string;
  amendment: string;
  affected_controls: number;
  cure_days: number;
};

export type AmendmentResponse = {
  id: string;
  correlation_id: string;
  status: string;
  risk_score: number;
  stages: string[];
  evidence_hash: string;
};

export type ErrorResponse = {
  code: string;
  message: string;
  correlation_id: string;
};

export const API_OPERATIONS = {
  submitAmendment: { method: "POST", path: "/api/v1/amendments" },
  listAmendments: { method: "GET", path: "/api/v1/amendments" },
  readAudit: { method: "GET", path: "/api/v1/audit" },
  replayAmendment: { method: "GET", path: "/api/v1/amendments/{correlation}" },
} as const;
