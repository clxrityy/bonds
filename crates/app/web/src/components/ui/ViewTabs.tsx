import type { ViewTab } from "../../lib/types";

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
					className={`inline-flex items-center gap-2 rounded-full border px-3 py-1.5 text-xs font-medium uppercase tracking-wide transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70 ${active === tab
							? "border-sky-400/70 bg-sky-500/10 text-sky-300"
							: "border-slate-700 bg-slate-800 text-slate-200 hover:bg-slate-700"
						}`}
					onClick={() => onChange(tab)}
				>
					<span>{tab}</span>
					<span className="text-[11px] text-slate-400">{counts[tab]}</span>
				</button>
			))}
		</div>
	);
}