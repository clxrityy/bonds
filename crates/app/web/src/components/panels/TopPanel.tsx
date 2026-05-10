import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ui, Icons } from "../../lib/ui";
import { Logo } from "../elements/Logo";

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
						<Logo />
					<button className={`${ui.ghostBtn} h-auto w-auto`} type="button" onClick={onSwitchToSide}>
						<Icons.SidePanelOpen />
						<span className="sr-only">Switch to side panel</span>
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