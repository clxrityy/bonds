import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ui } from "../../lib/ui";

type TopPanelProps = PanelControlsProps & {
	onSwitchToSide: () => void;
};

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
					<button className={ui.ghostBtn} type="button">
						Home
					</button>
					<button className={ui.ghostBtn} type="button" onClick={onSwitchToSide}>
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