import { useCallback, useMemo, useState } from "react";
import { AppShell } from "./components/layout/AppShell";
import { MainViewport } from "./components/layout/MainViewport";
import { BondViewer } from "./components/bonds/BondViewer";
import { PanelHost } from "./components/panels/PanelHost";
import { DeleteBondDialog } from "./components/dialogs/DeleteBondDialog";
import { EditBondDialog } from "./components/dialogs/EditBondDialog";
import { HistoryPanel } from "./components/panels/HistoryPanel";
import { RestoreConfirmDialog } from "./components/dialogs/RestoreConfirmDialog";
import { useBonds } from "./hooks/useBonds";
import { useSnapshots } from "./hooks/useSnapshots";
import { usePanelLayout } from "./hooks/usePanelLayout";
import { useResizableSidePanel } from "./hooks/useResizableSidePanel";
import { useBondFilters } from "./hooks/useBondFilters";
import type {
	BondDetailItem,
	BondListItem,
	BondMetadata,
	SnapshotItem,
} from "./lib/types";

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

	// History panel state.
	const [historyBond, setHistoryBond] = useState<BondListItem | null>(null);
	const [restoreCandidate, setRestoreCandidate] = useState<SnapshotItem | null>(null);

	const {
		snapshots,
		loading: historyLoading,
		error: historyError,
		refresh: refreshHistory,
		createNow: createSnapshotNow,
		creating: historyCreating,
		restoring: historyRestoring,
		actionError: historyActionError,
		clearActionError: clearHistoryActionError,
		restore: restoreSnapshot,
	} = useSnapshots({
		bondId: historyBond?.id ?? null,
		enabled: Boolean(historyBond),
	});

	const actionBusy = useMemo(
		() => updating || deleting || historyCreating || historyRestoring,
		[updating, deleting, historyCreating, historyRestoring],
	);

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

	const openHistory = useCallback(
		(bond: BondListItem) => {
			// Clear any stale action errors from previous history operations.
			clearHistoryActionError();
			setRestoreCandidate(null);
			setHistoryBond(bond);
		},
		[clearHistoryActionError],
	);

	const closeHistory = useCallback(() => {
		clearHistoryActionError();
		setRestoreCandidate(null);
		setHistoryBond(null);
	}, [clearHistoryActionError]);

	const requestRestore = useCallback(
		(snapshot: SnapshotItem) => {
			clearHistoryActionError();
			setRestoreCandidate(snapshot);
		},
		[clearHistoryActionError],
	);

	const cancelRestore = useCallback(() => {
		clearHistoryActionError();
		setRestoreCandidate(null);
	}, [clearHistoryActionError]);

	const confirmRestore = useCallback(async () => {
		if (!historyBond || !restoreCandidate) return;

		await restoreSnapshot(restoreCandidate.id);
		setRestoreCandidate(null);

		// Keep bond list status in sync after restore.
		await refresh();
	}, [historyBond, restoreCandidate, restoreSnapshot, refresh]);

	const confirmDelete = useCallback(async () => {
		if (!deletingBond) return;

		const deletedId = deletingBond.id;

		// Safe mode for this phase: do not remove underlying real target paths.
		await remove({
			id: deletedId,
			withTarget: false,
		});

		setDeletingBond(null);

		// If the deleted bond is currently open in history, close that panel.
		if (historyBond?.id === deletedId) {
			closeHistory();
		}
	}, [deletingBond, remove, historyBond, closeHistory]);

	const handleCreateSnapshot = useCallback(async () => {
		await createSnapshotNow();
	}, [createSnapshotNow]);

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
					onHistoryBond={openHistory}
					onEditBond={openEdit}
					onDeleteBond={openDelete}
					actionBusy={actionBusy}
				/>
			</MainViewport>

			<HistoryPanel
				open={Boolean(historyBond)}
				bond={historyBond}
				snapshots={snapshots}
				loading={historyLoading}
				error={historyError}
				creating={historyCreating}
				restoring={historyRestoring}
				actionError={historyActionError}
				onClose={closeHistory}
				onRefresh={refreshHistory}
				onCreateSnapshot={handleCreateSnapshot}
				onRequestRestore={requestRestore}
			/>

			<RestoreConfirmDialog
				open={Boolean(historyBond && restoreCandidate)}
				bond={historyBond}
				snapshot={restoreCandidate}
				restoring={historyRestoring}
				restoreError={historyActionError}
				onCancel={cancelRestore}
				onConfirm={confirmRestore}
			/>

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