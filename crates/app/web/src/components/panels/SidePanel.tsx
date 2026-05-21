import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ResizeHandle } from "./ResizeHandle";
import { ui } from "../../lib/ui";
import { Logo } from "../elements/Logo";
import { TopPanelOpenIcon } from "../ui/Icons";

type SidePanelProps = PanelControlsProps & {
	width: number;
	onSwitchToTop: () => void;
	onResizePointerDown: (event: React.PointerEvent<HTMLDivElement>) => void;
};

export function SidePanel({
	width,
	search,
	activeTab,
	counts,
	onSearchChange,
	onTabChange,
	onSwitchToTop,
	onResizePointerDown,
}: SidePanelProps) {
	return (
		<aside className="relative grid h-full grid-rows-[auto_auto_1fr] gap-2 border border-zinc-700 bg-zinc-300 p-2 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]" style={{ width }}>
			<div className="flex items-center justify-between gap-2 border-b border-zinc-700 pb-2">
				<Logo />
				<button className={ui.iconToolBtn} type="button" onClick={onSwitchToTop}>
					<TopPanelOpenIcon className="fill-zinc-700" />
					<span className="sr-only">Switch to top panel</span>
				</button>
			</div>

			<div className="grid gap-2 border border-zinc-700 bg-zinc-250 p-2 shadow-[inset_1px_1px_0_rgba(0,0,0,0.18),inset_-1px_-1px_0_rgba(255,255,255,0.65)]">
				<SearchInput value={search} onChange={onSearchChange} />
				<ViewTabs active={activeTab} counts={counts} onChange={onTabChange} />
			</div>

			<ResizeHandle onPointerDown={onResizePointerDown} />
		</aside>
	);
}