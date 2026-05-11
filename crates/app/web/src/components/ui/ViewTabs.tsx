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
		<div className="inline-flex flex-wrap items-center gap-1 border border-zinc-700 bg-zinc-300 p-1 shadow-[inset_1px_1px_0_rgba(255,255,255,0.72),inset_-1px_-1px_0_rgba(0,0,0,0.2)]" role="tablist" aria-label="Bond status filter">
			{TABS.map((tab) => (
				<button
					key={tab}
					type="button"
					role="tab"
					aria-selected={active === tab}
					className={cx(ui.tabBase, active ? ui.tabActive : ui.tabIdle)}
					onClick={() => onChange(tab)}
				>
					<span className="leading-none">{tab}</span>
					<span className={ui.tabCount}>{counts[tab]}</span>
				</button>
			))}
		</div>
	);
}