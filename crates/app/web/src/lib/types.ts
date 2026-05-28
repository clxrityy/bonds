// Shared domain types used across components/hooks.

export type BondStatus = "healthy" | "warning" | "broken";
export type PanelMode = "side" | "top";
export type ViewTab = "all" | BondStatus;

/**
 * Metadata is represented as a simple key/value map.
 * Keeping this shape mirrors the Rust HashMap<String, String>.
 */
export type BondMetadata = Record<string, string>;

export interface BondListItem {
	id: string;
	name: string | null;
	source: string;
	target: string;
	createdAt: string;
	status: BondStatus;
	metadataCount: number;
}

export interface BondDetailItem {
	id: string;
	name: string | null;
	source: string;
	target: string;
	createdAt: string;
	status: BondStatus;
	metadata: BondMetadata | null;
}

export interface LayoutState {
	mode: PanelMode;
	isOpen: boolean;
	sideWidth: number;
	search: string;
	activeTab: ViewTab;
}

export interface CreateBondInput {
	source: string;
	target?: string | null;
	name?: string | null;
}

export interface UpdateBondInput {
	id: string;
	source?: string | null;
	target?: string | null;
	name?: string | null;
}

export interface UpdateBondMetadataInput {
	id: string;
	metadata: BondMetadata | null;
}

export interface DeleteBondInput {
	id: string;
	withTarget?: boolean;
}