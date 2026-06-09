import type { PanelControlsProps } from "./panelTypes";
import { SearchInput } from "../ui/SearchInput";
import { ViewTabs } from "../ui/ViewTabs";
import { ui } from "../../lib/ui";
import { Logo } from "../elements/Logo";
import { SidePanelOpenIcon } from "../ui/Icons";

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
		<section className="grid gap-2 border border-zinc-700 bg-zinc-300 p-2 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]">
			<div className="flex items-center justify-between gap-2 border-b border-zinc-700 pb-2">
				<div className="flex items-center gap-1.5">
						<Logo />
					<button className={ui.iconToolBtn} type="button" onClick={onSwitchToSide}>
						<SidePanelOpenIcon className="fill-zinc-700" />
						<span className="sr-only">Switch to side panel</span>
					</button>
				</div>

				<div className="w-full max-w-[18rem]">
					<SearchInput
						value={search}
						onChange={onSearchChange}
						placeholder="Search bonds + metadata (meta:key=value)"
					/>
				</div>
			</div>

			<ViewTabs active={activeTab} counts={counts} onChange={onTabChange} />
		</section>
	);
}