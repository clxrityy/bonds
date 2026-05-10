/**
 * Shared Tailwind utility tokens.
 * This keeps styling consistent and avoids repeating large class strings.
 */

export function cx(...parts: (string | false | null | undefined)[]): string {
	return parts.filter(Boolean).join(" ");
}

export const ui = {
	// Generic surfaces
	panelSurface: "border-slate-700/70 bg-slate-900/70 backdrop-blur",

	// Buttons
	chromeBtnBase:
		"rounded-md border px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70",
	chromeBtnIdle: "border-slate-700 bg-slate-800 text-slate-200 hover:bg-slate-700",
	chromeBtnActive: "border-sky-400/70 bg-sky-500/10 text-sky-300",

	ghostBtn:
		"rounded-md border border-slate-700 bg-slate-800 px-3 py-2 text-sm font-medium text-slate-100 transition-colors hover:bg-slate-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70",

	primaryBtn:
		"rounded-md border border-sky-400/50 bg-sky-500/15 px-3 py-2 text-sm font-medium text-sky-200 transition-colors hover:bg-sky-500/25 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70",

	// Inputs
	searchInput:
		"w-full rounded-md border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70",

	// Tabs
	tabBase:
		"inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium uppercase tracking-wide transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70",
	tabIdle: "border-slate-700 bg-slate-800 text-slate-200 hover:bg-slate-700",
	tabActive: "border-sky-400/70 bg-sky-500/10 text-sky-300",
	tabCount: "text-[11px] text-slate-400",

	// Cards / states
	card: "rounded-sm border border-slate-700 bg-slate-900 p-3",
	stateCard: "rounded-sm border border-dashed border-slate-700 bg-slate-900 p-4 text-slate-200",
	stateCardError: "border-rose-500/50",
	statusPillBase:
		"rounded-full border px-2 py-1 text-[11px] font-medium uppercase tracking-wide",
} as const;

import { LuPanelRightClose, LuPanelLeftOpen, LuPanelTopOpen, LuPanelTopClose, LuHouse, LuRefreshCw } from "react-icons/lu";

export const Icons = {
	SidePanelClose: LuPanelRightClose,
	SidePanelOpen: LuPanelLeftOpen,
	TopPanelOpen: LuPanelTopOpen,
	TopPanelClose: LuPanelTopClose,
	Home: LuHouse,
	Refresh: LuRefreshCw,
} as const;