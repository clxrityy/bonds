import type { ViewTab } from "../../lib/types";

export interface PanelControlsProps {
	search: string;
	activeTab: ViewTab;
	counts: Record<ViewTab, number>;
	onSearchChange: (value: string) => void;
	onTabChange: (tab: ViewTab) => void;
};