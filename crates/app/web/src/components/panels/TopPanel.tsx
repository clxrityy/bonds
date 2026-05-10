import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";

type TopPanelProps = PanelControlsProps & {
	onSwitchToSide: () => void;
};

const ghostBtn =
	"rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm font-medium text-slate-100 transition-colors hover:bg-slate-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70";

export function TopPanel({
	search,
	activeTab,
	counts,
	onSearchChange,
	onTabChange,
	onSwitchToSide,
}: TopPanelProps) {
	return (
		<section className="grid gap-3">
			<div className="flex flex-wrap items-center justify-between gap-3">
				<div className="flex items-center gap-2">
					<button className={ghostBtn} type="button">
						Home
					</button>
					<button className={ghostBtn} type="button" onClick={onSwitchToSide}>
						Use side panel
					</button>
				</div>

				<div className="w-full max-w-sm">
					<SearchInput value={search} onChange={onSearchChange} />
				</div>
			</div>

			<ViewTabs active={activeTab} counts={counts} onChange={onTabChange} />
		</section>
	);
}