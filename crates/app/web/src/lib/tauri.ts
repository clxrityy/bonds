import { invoke } from "@tauri-apps/api/core";
import type {
	BondDetailItem,
	BondListItem,
	CreateBondInput,
	DeleteBondInput,
	UpdateBondInput,
	UpdateBondMetadataInput,
} from "./types";

// Typed wrapper for Tauri command so only this file knows command names.
export async function listBonds(): Promise<BondListItem[]> {
	return invoke<BondListItem[]>("list_bonds");
}

export async function getBondDetail(id: string): Promise<BondDetailItem> {
	return invoke<BondDetailItem>("get_bond_detail", {
		request: { id },
	});
}

export async function createBond(input: CreateBondInput): Promise<BondListItem> {
	return invoke<BondListItem>("create_bond", {
		// Match Rust request field names (camelCase via serde rename_all).
		request: {
			source: input.source,
			target: input.target ?? null,
			name: input.name ?? null,
		},
	});
}

export async function updateBond(input: UpdateBondInput): Promise<BondListItem> {
	return invoke<BondListItem>("update_bond", {
		request: {
			id: input.id,
			source: input.source ?? null,
			target: input.target ?? null,
			name: input.name ?? null,
		},
	});
}

export async function updateBondMetadata(
	input: UpdateBondMetadataInput,
): Promise<BondDetailItem> {
	return invoke<BondDetailItem>("update_bond_metadata", {
		request: {
			id: input.id,
			// Send null explicitly when cleared so Rust can normalize to None.
			metadata: input.metadata ?? null,
		},
	});
}

export async function deleteBond(input: DeleteBondInput): Promise<BondListItem> {
	return invoke<BondListItem>("delete_bond", {
		request: {
			id: input.id,
			// Safe default for this phase: never delete underlying real targets from UI.
			withTarget: input.withTarget ?? false,
		},
	});
}