import type { Program } from "https://esm.sh/v130/@swc/wasm@1.3.76";
import { instantiate } from "./lib/deno_ast_wasm.generated.js";

// deno-lint-ignore no-explicit-any
let parseSyncFunc: Promise<(s: string) => any> | null = null;

export interface ParsedSource {
  program: Program;
  comments: [{ text: string; span_lo: number; span_hi: number }];
}
export async function parse(
  source: string,
): Promise<ParsedSource> {
  parseSyncFunc ??= instantiate().then((mod) => mod.parseSync);
  return (await parseSyncFunc)(source);
}
