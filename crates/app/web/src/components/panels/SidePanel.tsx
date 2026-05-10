import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ResizeHandle } from "./ResizeHandle";
import { ui } from "../../lib/ui";

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
		<aside className="relative grid h-full grid-rows-[auto_auto_1fr] gap-3 p-3" style={{ width }}>
			<div className="flex flex-wrap items-center gap-2">
				<button className={ui.ghostBtn} type="button">
					Home
				</button>
				<button className={ui.ghostBtn} type="button" onClick={onSwitchToTop}>
					Use top panel
				</button>
			</div>

			<div className="grid gap-3">
				<SearchInput value={search} onChange={onSearchChange} />
				<ViewTabs active={activeTab} counts={counts} onChange={onTabChange} />
			</div>

			<ResizeHandle onPointerDown={onResizePointerDown} />
		</aside>
	);
}