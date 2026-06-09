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
	// Included in list payload for metadata-aware search/filtering.
	metadata: BondMetadata | null;
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

/**
 * History snapshot item returned from app backend.
 * Field names match Rust's `#[serde(rename_all = "camelCase")]` output.
 */
export interface SnapshotItem {
	id: string;
	bondId: string;
	bondName: string | null;
	createdAt: string;
	sourcePath: string;
	targetPath: string;
	storagePath: string;
	fileCount: number;
	bytesTotal: number;
	metadataCount: number;
}

export interface RestoreSnapshotInput {
	id: string;
	snapshotId: string;
}

export interface DeleteSnapshotInput {
	id: string;
	snapshotId: string;
}

export interface RestoreSnapshotResult {
	id: string;
	bondId: string;
	snapshotId: string;
	safetySnapshotId: string | null;
	createdAt: string;
	status: string;
	notes: string | null;
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