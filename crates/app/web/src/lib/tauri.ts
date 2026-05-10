import { invoke } from "@tauri-apps/api/core";
import type { BondListItem } from "./types";

// Typed wrapper for Tauri command so only this file knows command names.
export async function listBonds(): Promise<BondListItem[]> {
	return invoke<BondListItem[]>("list_bonds");
}