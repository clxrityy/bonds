import { invoke } from "@tauri-apps/api/core";
import type { BondListItem, CreateBondInput } from "./types";

// Typed wrapper for Tauri command so only this file knows command names.
export async function listBonds(): Promise<BondListItem[]> {
	return invoke<BondListItem[]>("list_bonds");
}

export async function createBond(input: CreateBondInput): Promise<BondListItem> {
	return invoke<BondListItem>("create_bond", {
		// Match Rust `CreateBondRequest` field names (camelCase via serde rename_all).
		request: {
			source: input.source,
			target: input.target ?? null,
			name: input.name ?? null,
		},
	});
}