import { AppShell } from "./components/layout/AppShell";
import { MainViewport } from "./components/layout/MainViewport";
import { BondViewer } from "./components/bonds/BondViewer";
import { PanelHost } from "./components/panels/PanelHost";
import { useBonds } from "./hooks/useBonds";
import { usePanelLayout } from "./hooks/usePanelLayout";
import { useResizableSidePanel } from "./hooks/useResizableSidePanel";
import { useBondFilters } from "./hooks/useBondFilters";

export default function App() {
	const { layout, isCompact, setMode, togglePanel, setSearch, setActiveTab, setSideWidth } =
		usePanelLayout();

	const { bonds, loading, error, refresh, create, creating, createError } = useBonds();
	const { filtered, counts } = useBondFilters(bonds, layout.search, layout.activeTab);

	const { onPointerDown } = useResizableSidePanel({
		enabled: layout.mode === "side" && layout.isOpen && !isCompact,
		currentWidth: layout.sideWidth,
		onWidthChange: setSideWidth,
	});

	const panelNode = (
		<PanelHost
			mode={layout.mode}
			sideWidth={layout.sideWidth}
			search={layout.search}
			activeTab={layout.activeTab}
			counts={counts}
			onSearchChange={setSearch}
			onTabChange={setActiveTab}
			onSwitchToTop={() => setMode("top")}
			onSwitchToSide={() => setMode("side")}
			onResizePointerDown={onPointerDown}
		/>
	);

	return (
		<AppShell
			mode={layout.mode}
			isOpen={layout.isOpen}
			isCompact={isCompact}
			sideWidth={layout.sideWidth}
			panel={panelNode}
			onTogglePanel={togglePanel}
			onSetMode={setMode}
		>
			<MainViewport>
				<BondViewer
					loading={loading}
					error={error}
					bonds={filtered}
					onRefresh={refresh}
					onCreate={create}
					creating={creating}
					createError={createError}
				/>
			</MainViewport>
		</AppShell>
	);
}