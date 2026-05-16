import type { CSSProperties, PropsWithChildren, ReactNode } from "react";
import type { PanelMode } from "../../lib/types";
import { cx, ui, Icons } from "../../lib/ui";

type AppShellProps = PropsWithChildren<{
	mode: PanelMode;
	isOpen: boolean;
	isCompact: boolean;
	sideWidth: number;
	panel: ReactNode;
	onTogglePanel: () => void;
	onSetMode: (mode: PanelMode) => void;
}>;

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
	const bodyStyle: CSSProperties | undefined =
		mode === "side" && isOpen && !isCompact
			? { gridTemplateColumns: `${sideWidth}px minmax(0, 1fr)` }
			: undefined;

	// Compact panel placement switches by current mode.
	const compactPanelClass = cx(
		"fixed z-50 border border-zinc-700 bg-zinc-300 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]",
		mode === "side"
			? "inset-y-0 left-0 w-[min(85vw,340px)] border-r"
			: "inset-x-0 top-0 border-b"
	);

	return (
		<div className="grid min-h-screen grid-rows-[auto_auto_1fr] bg-zinc-400 text-zinc-900">
			<header className="flex items-center justify-between gap-2 border border-zinc-700 bg-zinc-300 px-3 py-2 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]">
				<button
					className={cx(ui.chromeBtnBase, ui.chromeBtnIdle)}
					type="button"
					aria-label={isOpen ? "Close panel" : "Open panel"}
					onClick={onTogglePanel}
				>
					{isOpen ? <Icons.SidePanelClose /> : <Icons.SidePanelOpen />}
				</button>

				<div className="flex items-center gap-2">
					<button
						type="button"
						className={cx(
							ui.chromeBtnBase,
							mode === "side" ? ui.chromeBtnActive : ui.chromeBtnIdle
						)}
						onClick={() => onSetMode("side")}
					>
						<Icons.SidePanelOpen />
						<span className="sr-only">Side panel mode</span>
					</button>
					<button
						type="button"
						className={cx(
							ui.chromeBtnBase,
							mode === "top" ? ui.chromeBtnActive : ui.chromeBtnIdle
						)}
						onClick={() => onSetMode("top")}
					>
						<Icons.TopPanelOpen />
						<span className="sr-only">Top panel mode</span>
					</button>
				</div>
			</header>

			{mode === "top" && isOpen && !isCompact ? (
				<div className={cx("border-b px-3 py-2", ui.panelSurface)}>{panel}</div>
			) : null}

			<div className="grid min-h-0 grid-cols-1" style={bodyStyle}>
				{mode === "side" && isOpen && !isCompact ? (
					<div className={cx("min-w-0 border-r", ui.panelSurface)}>{panel}</div>
				) : null}

				<main className="min-h-0 min-w-0">{children}</main>
			</div>

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
