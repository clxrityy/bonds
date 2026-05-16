// Shared domain types used across components/hooks.

export type BondStatus = "healthy" | "warning" | "broken";
export type PanelMode = "side" | "top";
export type ViewTab = "all" | BondStatus;

export interface BondListItem {
	id: string;
	name: string | null;
	source: string;
	target: string;
	createdAt: string;
	status: BondStatus;
	metadataCount: number;
};

export interface LayoutState {
	mode: PanelMode;
	isOpen: boolean;
	sideWidth: number;
	search: string;
	activeTab: ViewTab;
};