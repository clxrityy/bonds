import { useCallback, useMemo, useState } from "react";
import { AppShell } from "./components/layout/AppShell";
import { MainViewport } from "./components/layout/MainViewport";
import { BondViewer } from "./components/bonds/BondViewer";
import { PanelHost } from "./components/panels/PanelHost";
import { DeleteBondDialog } from "./components/dialogs/DeleteBondDialog";
import { EditBondDialog } from "./components/dialogs/EditBondDialog";
import { useBonds } from "./hooks/useBonds";
import { usePanelLayout } from "./hooks/usePanelLayout";
import { useResizableSidePanel } from "./hooks/useResizableSidePanel";
import { useBondFilters } from "./hooks/useBondFilters";
import type { BondDetailItem, BondListItem, BondMetadata } from "./lib/types";

function normalizeMetadata(input: BondMetadata | null): BondMetadata | null {
	if (!input) return null;
	return Object.keys(input).length > 0 ? input : null;
}

function stableMetadataString(input: BondMetadata | null): string {
	const normalized = normalizeMetadata(input);
	if (!normalized) return "null";

	// Stable key ordering so deep equality is deterministic.
	const sorted = Object.keys(normalized)
		.sort()
		.reduce((acc, key) => {
			acc[key] = normalized[key];
			return acc;
		}, {} as BondMetadata);

	return JSON.stringify(sorted);
}

export default function App() {
	const { layout, isCompact, setMode, togglePanel, setSearch, setActiveTab, setSideWidth } =
		usePanelLayout();

	const {
		bonds,
		loading,
		error,
		refresh,
		create,
		creating,
		createError,
		getDetail,
		update,
		saveMetadata,
		updating,
		updateError,
		remove,
		deleting,
		deleteError,
	} = useBonds();

	const { filtered, counts } = useBondFilters(bonds, layout.search, layout.activeTab);

	const { onPointerDown } = useResizableSidePanel({
		enabled: layout.mode === "side" && layout.isOpen && !isCompact,
		currentWidth: layout.sideWidth,
		onWidthChange: setSideWidth,
	});

	// Edit dialog state.
	const [editingBond, setEditingBond] = useState<BondListItem | null>(null);
	const [editDetail, setEditDetail] = useState<BondDetailItem | null>(null);
	const [editDetailLoading, setEditDetailLoading] = useState(false);
	const [editDetailError, setEditDetailError] = useState<string | null>(null);

	// Delete dialog state.
	const [deletingBond, setDeletingBond] = useState<BondListItem | null>(null);

	const actionBusy = useMemo(() => updating || deleting, [updating, deleting]);

	const openEdit = useCallback(
		async (bond: BondListItem) => {
			setEditingBond(bond);
			setEditDetail(null);
			setEditDetailError(null);
			setEditDetailLoading(true);

			try {
				const detail = await getDetail(bond.id);
				setEditDetail(detail);
			} catch (err) {
				setEditDetailError(err instanceof Error ? err.message : String(err));
			} finally {
				setEditDetailLoading(false);
			}
		},
		[getDetail],
	);

	const closeEdit = useCallback(() => {
		setEditingBond(null);
		setEditDetail(null);
		setEditDetailError(null);
		setEditDetailLoading(false);
	}, []);

	const saveEdit = useCallback(
		async (input: {
			id: string;
			source: string;
			target: string;
			name: string | null;
			metadata: BondMetadata | null;
		}) => {
			if (!editDetail) return;

			const nextName = input.name ?? null;
			const prevName = editDetail.name ?? null;

			// `update_bond` requires at least one of source/target/name changed.
			const coreChanged =
				input.source !== editDetail.source ||
				input.target !== editDetail.target ||
				nextName !== prevName;

			if (coreChanged) {
				await update({
					id: input.id,
					source: input.source,
					target: input.target,
					name: nextName,
				});
			}

			const nextMetadata = normalizeMetadata(input.metadata);
			const prevMetadata = normalizeMetadata(editDetail.metadata);

			if (stableMetadataString(nextMetadata) !== stableMetadataString(prevMetadata)) {
				await saveMetadata({
					id: input.id,
					metadata: nextMetadata,
				});
			}

			// Close only after successful save operations.
			closeEdit();
		},
		[editDetail, update, saveMetadata, closeEdit],
	);

	const openDelete = useCallback((bond: BondListItem) => {
		setDeletingBond(bond);
	}, []);

	const closeDelete = useCallback(() => {
		setDeletingBond(null);
	}, []);

	const confirmDelete = useCallback(async () => {
		if (!deletingBond) return;

		// Safe mode for this phase: do not remove underlying real target paths.
		await remove({
			id: deletingBond.id,
			withTarget: false,
		});

		setDeletingBond(null);
	}, [deletingBond, remove]);

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
					onEditBond={openEdit}
					onDeleteBond={openDelete}
					actionBusy={actionBusy}
				/>
			</MainViewport>

			<EditBondDialog
				open={Boolean(editingBond)}
				detail={editDetail}
				loading={editDetailLoading}
				loadError={editDetailError}
				saving={updating}
				saveError={updateError}
				onClose={closeEdit}
				onSave={saveEdit}
			/>

			<DeleteBondDialog
				open={Boolean(deletingBond)}
				bond={deletingBond}
				deleting={deleting}
				deleteError={deleteError}
				onCancel={closeDelete}
				onConfirm={confirmDelete}
			/>
		</AppShell>
	);
}