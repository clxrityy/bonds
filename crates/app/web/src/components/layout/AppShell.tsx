import type { CSSProperties, PropsWithChildren, ReactNode } from "react";
import type { PanelMode } from "../../lib/types";

type AppShellProps = PropsWithChildren<{
	mode: PanelMode;
	isOpen: boolean;
	isCompact: boolean;
	sideWidth: number;
	panel: ReactNode;
	onTogglePanel: () => void;
	onSetMode: (mode: PanelMode) => void;
}>;

const chromeBtnBase =
	"rounded-lg border px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70";

export function AppShell({
	mode,
	isOpen,
	isCompact,
	sideWidth,
	panel,
	onTogglePanel,
	onSetMode,
	children,
}: AppShellProps) {
	// Dynamic grid width only when side panel is docked.
	const bodyStyle: CSSProperties | undefined =
		mode === "side" && isOpen && !isCompact
			? { gridTemplateColumns: `${sideWidth}px minmax(0, 1fr)` }
			: undefined;

	// Compact overlay variant based on active mode.
	const compactPanelClass =
		mode === "side"
			? "fixed inset-y-0 left-0 z-50 w-[min(85vw,340px)] border-r border-slate-700/70 bg-slate-900/95 shadow-2xl backdrop-blur"
			: "fixed inset-x-0 top-0 z-50 border-b border-slate-700/70 bg-slate-900/95 shadow-2xl backdrop-blur";

	return (
		<div className="grid min-h-screen grid-rows-[auto_auto_1fr] bg-slate-950 text-slate-100">
			<header className="flex items-center justify-between gap-3 border-b border-slate-700/70 bg-slate-900/70 px-4 py-3 backdrop-blur">
				<button
					className={`${chromeBtnBase} border-slate-700 bg-slate-800 text-slate-100 hover:bg-slate-700`}
					type="button"
					onClick={onTogglePanel}
				>
					{isOpen ? "Hide panel" : "Show panel"}
				</button>

				<div className="flex items-center gap-2">
					<button
						type="button"
						className={`${chromeBtnBase} ${mode === "side"
								? "border-sky-400/70 bg-sky-500/10 text-sky-300"
								: "border-slate-700 bg-slate-800 text-slate-200 hover:bg-slate-700"
							}`}
						onClick={() => onSetMode("side")}
					>
						Side
					</button>
					<button
						type="button"
						className={`${chromeBtnBase} ${mode === "top"
								? "border-sky-400/70 bg-sky-500/10 text-sky-300"
								: "border-slate-700 bg-slate-800 text-slate-200 hover:bg-slate-700"
							}`}
						onClick={() => onSetMode("top")}
					>
						Top
					</button>
				</div>
			</header>

			{/* Docked top panel (wide mode only) */}
			{mode === "top" && isOpen && !isCompact ? (
				<div className="border-b border-slate-700/70 bg-slate-900/70 px-3 py-2 backdrop-blur">
					{panel}
				</div>
			) : null}

			<div className="grid min-h-0 grid-cols-1" style={bodyStyle}>
				{/* Docked side panel (wide mode only) */}
				{mode === "side" && isOpen && !isCompact ? (
					<div className="min-w-0 border-r border-slate-700/70 bg-slate-900/70 backdrop-blur">
						{panel}
					</div>
				) : null}

				<main className="min-h-0 min-w-0">{children}</main>
			</div>

			{/* Overlay panel on compact viewports */}
			{isCompact && isOpen ? (
				<>
					<button
						type="button"
						className="fixed inset-0 z-40 bg-black/50"
						aria-label="Close panel overlay"
						onClick={onTogglePanel}
					/>
					<div className={compactPanelClass}>{panel}</div>
				</>
			) : null}
		</div>
	);
}