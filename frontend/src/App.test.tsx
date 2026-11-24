import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
describe("ClauseForge studio",()=>{
  it("renders a domain-specific amendment composer",()=>{
    vi.stubGlobal("fetch",vi.fn().mockResolvedValue({ok:true,json:async()=>[]}));
    render(<App/>);
    expect(screen.getByRole("heading",{name:/trace the clause change/i})).toBeInTheDocument();
    expect(screen.getByRole("textbox",{name:/ai supplier/i})).toHaveValue("VectorCloud AI");
    expect(screen.getByRole("button",{name:/analyze amendment impact/i})).toBeInTheDocument();
    expect(screen.getByRole("form",{name:/amendment composer/i})).toBeInTheDocument();
  });
});
