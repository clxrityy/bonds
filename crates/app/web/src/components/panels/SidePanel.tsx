import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ResizeHandle } from "./ResizeHandle";

type SidePanelProps = PanelControlsProps & {
	width: number;
	onSwitchToTop: () => void;
	onResizePointerDown: (event: React.PointerEvent<HTMLDivElement>) => void;
};

const ghostBtn =
	"rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm font-medium text-slate-100 transition-colors hover:bg-slate-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70";

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
		<aside
			className="relative grid h-full grid-rows-[auto_auto_1fr] gap-3 p-3"
			style={{ width }}
		>
			<div className="flex flex-wrap items-center gap-2">
				<button className={ghostBtn} type="button">
					Home
				</button>
				<button className={ghostBtn} type="button" onClick={onSwitchToTop}>
					Use top panel
				</button>
			</div>

			<div className="grid gap-3">
				<SearchInput value={search} onChange={onSearchChange} />
				<ViewTabs active={activeTab} counts={counts} onChange={onTabChange} />
			</div>

			{/* Vertical drag handle for side width resize */}
			<ResizeHandle onPointerDown={onResizePointerDown} />
		</aside>
	);
}