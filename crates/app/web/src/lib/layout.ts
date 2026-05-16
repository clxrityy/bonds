import type { LayoutState } from "./types";

// Local storage key versioned so future shape changes are easy.
export const LAYOUT_STORAGE_KEY = "bonds.app.layout.v1";

// Breakpoint where panel becomes overlay instead of docked.
export const COMPACT_BREAKPOINT = 980;

// Side panel width constraints.
export const SIDE_WIDTH_MIN = 220;
export const SIDE_WIDTH_MAX = 420;

export const DEFAULT_LAYOUT: LayoutState = {
	mode: "side",
	isOpen: true,
	sideWidth: 280,
	search: "",
	activeTab: "all",
};

export function clampSideWidth(width: number): number {
	return Math.min(SIDE_WIDTH_MAX, Math.max(SIDE_WIDTH_MIN, width));
}