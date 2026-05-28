/**
 * Shared Tailwind utility tokens.
 * Retro/vintage grayscale style system:
 * - flat/boxed controls
 * - raised/inset edges
 * - minimal color accents
 */

/**
 * Utility for conditionally joining class names together, filtering out falsy values.
 * Example usage: cx("base-class", isActive && "active-class", isDisabled ? "disabled-class" : null)
 * This will include "active-class" only if isActive is true, and "disabled-class" only if isDisabled is true.
 * Falsy values (false, null, undefined) will be filtered out and not included in the final class string.
 */
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
		"inline-flex items-center gap-1.5 rounded-[4px] border border-zinc-700 bg-zinc-200 px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100 active:translate-y-px justify-center disabled:pointer-events-none disabled:opacity-50",
	iconToolBtn: "inline-flex h-7 w-7 items-center justify-center rounded-[3px] border border-zinc-700 bg-zinc-200 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100 active:translate-y-px",

	// Keep command button darker for hierarchy, but still grayscale.
	primaryBtn:
		"inline-flex items-center gap-1.5 rounded-[4px] border border-zinc-800 bg-zinc-900 px-2.5 py-1.5 text-[11px] font-semibold uppercase tracking-wide text-zinc-100 shadow-[inset_1px_1px_0_rgba(255,255,255,0.12),inset_-1px_-1px_0_rgba(0,0,0,0.35)] hover:bg-zinc-800 active:translate-y-px disabled:pointer-events-none disabled:opacity-50",

	// Inset field style (old desktop utility look)
	searchInput:
		"w-full rounded-[2px] border border-zinc-700 bg-zinc-100 px-2 py-1.5 text-[13px] leading-5 text-zinc-900 placeholder:text-zinc-500 shadow-[inset_1px_1px_0_rgba(0,0,0,0.22),inset_-1px_-1px_0_rgba(255,255,255,0.75)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-900",
	// Input with more padding for larger forms, but same style.
	formInput:
		"w-full rounded-[2px] border border-zinc-700 bg-zinc-100 px-3 py-2 text-[13px] leading-5 text-zinc-900 placeholder:text-zinc-500 shadow-[inset_1px_1px_0_rgba(0,0,0,0.22),inset_-1px_-1px_0_rgba(255,255,255,0.75)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-900",

	// Segmented tabs (not modern pills)
	tabBase:
		"inline-flex items-center gap-1 rounded-[2px] border px-2 py-1 text-[10px] font-semibold uppercase tracking-[0.08em] transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-zinc-900",
	tabIdle:
		"border-zinc-700 bg-zinc-200 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)] hover:bg-zinc-100",
	tabActive:
		"border-zinc-900 bg-zinc-50 text-zinc-950 shadow-[inset_1px_1px_0_rgba(255,255,255,0.85),inset_-1px_-1px_0_rgba(0,0,0,0.28)]",
	tabCount: "text-[9px] text-zinc-600",

	// Content panes
	card:
		"rounded-[4px] border border-zinc-700 bg-zinc-200 p-3 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.7),inset_-1px_-1px_0_rgba(0,0,0,0.18)]",
	stateCard:
		"rounded-[4px] border border-zinc-700 bg-zinc-200 p-4 text-zinc-900 shadow-[inset_1px_1px_0_rgba(255,255,255,0.7),inset_-1px_-1px_0_rgba(0,0,0,0.18)]",
	stateCardError: "border-zinc-800 bg-zinc-300",
	statusPillBase:
		"rounded-[4px] border px-2 py-1 text-[10px] font-semibold uppercase tracking-wide",
} as const;