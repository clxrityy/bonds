import type { ViewTab } from "../../lib/types";
import { cx, ui } from "../../lib/ui";

const TABS: ViewTab[] = ["all", "healthy", "warning", "broken"];

interface ViewTabsProps {
	active: ViewTab;
	counts: Record<ViewTab, number>;
	onChange: (tab: ViewTab) => void;
}

export function ViewTabs({ active, counts, onChange }: ViewTabsProps) {
	return (
		<div className="flex flex-wrap items-center gap-2" role="tablist" aria-label="Bond status filter">
			{TABS.map((tab) => (
				<button
					key={tab}
					type="button"
					role="tab"
					aria-selected={active === tab}
					className={cx(ui.tabBase, active === tab ? ui.tabActive : ui.tabIdle)}
					onClick={() => onChange(tab)}
				>
					<span>{tab}</span>
					<span className={ui.tabCount}>{counts[tab]}</span>
				</button>
			))}
		</div>
	);
}