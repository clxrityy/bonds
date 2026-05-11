/**
 * Shared Tailwind utility tokens.
 * Retro/vintage grayscale style system:
 * - flat/boxed controls
 * - raised/inset edges
 * - minimal color accents
 */

import {
	LuPanelRightClose,
	LuPanelLeftOpen,
	LuPanelTopOpen,
	LuPanelTopClose,
	LuHouse,
	LuRefreshCw,
} from "react-icons/lu";

export function cx(...parts: (string | false | null | undefined)[]): string {
	return parts.filter(Boolean).join(" ");
}

export const ui = {
	// Generic window/panel surface (retro frame)
	panelSurface:
		"border border-zinc-700 bg-zinc-300 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]",

	// Toolbar/chrome buttons
	chromeBtnBase:
		"inline-flex items-center gap-1.5 rounded-[4px] border px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-800",
	chromeBtnIdle:
		"border-zinc-700 bg-zinc-200 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100 active:translate-y-px active:shadow-[inset_1px_1px_0_rgba(0,0,0,0.2)]",
	chromeBtnActive:
		"border-zinc-800 bg-zinc-100 text-zinc-950 shadow-[inset_1px_1px_0_rgba(255,255,255,0.78),inset_-1px_-1px_0_rgba(0,0,0,0.24)]",

	ghostBtn:
		"inline-flex items-center gap-1.5 rounded-[4px] border border-zinc-700 bg-zinc-200 px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100 active:translate-y-px",

	// Keep command button darker for hierarchy, but still grayscale.
	primaryBtn:
		"inline-flex items-center gap-1.5 rounded-[4px] border border-zinc-800 bg-zinc-900 px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide text-zinc-100 shadow-[inset_1px_1px_0_rgba(255,255,255,0.12),inset_-1px_-1px_0_rgba(0,0,0,0.35)] hover:bg-zinc-800 active:translate-y-px",

	// Inset field style (old desktop utility look)
	searchInput:
		"w-full rounded-[3px] border border-zinc-700 bg-zinc-50 px-2.5 py-1.5 text-sm text-zinc-900 placeholder:text-zinc-500 shadow-[inset_1px_1px_0_rgba(0,0,0,0.2),inset_-1px_-1px_0_rgba(255,255,255,0.58)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-800",

	// Segmented tabs (not modern pills)
	tabBase:
		"inline-flex items-center gap-2 rounded-[4px] border px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-800",
	tabIdle:
		"border-zinc-700 bg-zinc-200 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100",
	tabActive:
		"border-zinc-900 bg-zinc-100 text-zinc-950 shadow-[inset_1px_1px_0_rgba(255,255,255,0.78),inset_-1px_-1px_0_rgba(0,0,0,0.24)]",
	tabCount: "text-[10px] text-zinc-600",

	// Content panes
	card:
		"rounded-[4px] border border-zinc-700 bg-zinc-200 p-3 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.7),inset_-1px_-1px_0_rgba(0,0,0,0.18)]",
	stateCard:
		"rounded-[4px] border border-zinc-700 bg-zinc-200 p-4 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.7),inset_-1px_-1px_0_rgba(0,0,0,0.18)]",
	stateCardError: "border-zinc-800 bg-zinc-300",
	statusPillBase:
		"rounded-[4px] border px-2 py-1 text-[10px] font-semibold uppercase tracking-wide",
} as const;

export const Icons = {
	SidePanelClose: LuPanelRightClose,
	SidePanelOpen: LuPanelLeftOpen,
	TopPanelOpen: LuPanelTopOpen,
	TopPanelClose: LuPanelTopClose,
	Home: LuHouse,
	Refresh: LuRefreshCw,
} as const;