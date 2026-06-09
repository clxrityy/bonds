import { formatBytes, formatDate, shortId } from "../../lib/format";
import type { BondListItem, SnapshotItem } from "../../lib/types";
import { cx, ui } from "../../lib/ui";
import { DeleteIcon, RefreshIcon, RestoreIcon, SnapshotIcon } from "../ui/Icons";

interface HistoryPanelProps {
	open: boolean;
	bond: BondListItem | null;
	snapshots: SnapshotItem[];
	loading: boolean;
	error: string | null;
	creating: boolean;
	restoring: boolean;
	deleting: boolean;
	actionError: string | null;
	onClose: () => void;
	onRefresh: () => Promise<void> | void;
	onCreateSnapshot: () => Promise<void> | void;
	onRequestRestore: (snapshot: SnapshotItem) => void;
	onRequestDelete: (snapshot: SnapshotItem) => void;
}

export function HistoryPanel({
	open,
	bond,
	snapshots,
	loading,
	error,
	creating,
	restoring,
	deleting,
	actionError,
	onClose,
	onRefresh,
	onCreateSnapshot,
	onRequestRestore,
	onRequestDelete,
}: HistoryPanelProps) {
	if (!open || !bond) return null;

	const busy = creating || restoring || deleting;

	return (
		<>
			<button
				type="button"
				className="fixed inset-0 z-40 bg-black/45"
				aria-label="Close history panel"
				onClick={onClose}
			/>

			<aside
				className={cx(
					ui.panelSurface,
					"fixed right-0 top-0 z-50 h-full w-full max-w-2xl overflow-y-auto p-4",
				)}
			>
				<div className="mb-3 flex items-start justify-between gap-3">
					<div>
						<h3 className="text-base font-semibold uppercase tracking-wide text-zinc-900">
							Bond history
						</h3>
						<p className="mt-1 text-xs text-slate-700 break-all">
							{bond.name ?? shortId(bond.id)} ({shortId(bond.id)})
						</p>
					</div>
					<button
						type="button"
						className={ui.ghostBtn}
						disabled={busy}
						onClick={onClose}
					>
						Close
					</button>
				</div>

				<div className={cx(ui.stateCard, "mb-3 p-3")}>
					<p className="text-[11px] uppercase tracking-wider text-slate-500 font-semibold">Source</p>
					<p className="mt-1 text-xs font-mono text-slate-800 break-all">{bond.source}</p>

					<p className="mt-3 text-[11px] uppercase tracking-wider text-slate-500 font-semibold">Target</p>
					<p className="mt-1 text-xs font-mono text-slate-800 break-all">{bond.target}</p>
				</div>

				<div className="mb-3 flex flex-wrap items-center gap-2">
					<button
						type="button"
						className={ui.ghostBtn}
						disabled={busy}
						onClick={() => void onRefresh()}
					>
						<RefreshIcon aria-label="Refresh history data" />
						<span className="sr-only">Refresh history data</span>
					</button>

					<button
						type="button"
						className={ui.primaryBtn}
						disabled={busy}
						onClick={() => void onCreateSnapshot()}
					>
						{creating ? "Creating…" : <span>
								<SnapshotIcon aria-label="Create snapshot" className="fill-slate-300" />
								<span className="sr-only">Create snapshot</span>
							</span>}
					</button>
					{restoring ? <span className="text-xs text-slate-600">Restore in progress…</span> : null}
					{deleting ? <span className="text-xs text-slate-600">Delete in progress…</span> : null}
				</div>

				{error ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mb-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">Unable to load snapshots.</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{error}
						</pre>
					</div>
				) : null}

				{actionError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mb-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">History action failed.</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{actionError}
						</pre>
					</div>
				) : null}

				{loading ? (
					<div className={ui.stateCard}>Loading snapshots…</div>
				) : snapshots.length === 0 ? (
					<div className={ui.stateCard}>
						<p className="text-slate-700">No snapshots yet.</p>
					</div>
				) : (
					<div className="grid gap-2">
						{snapshots.map((snapshot) => (
							<article key={snapshot.id} className={cx(ui.card, "p-3")}>
								<div className="flex flex-wrap items-start justify-between gap-2">
									<div>
										<p className="text-xs font-semibold uppercase tracking-wide text-slate-700">
											{shortId(snapshot.id)}
										</p>
										<p className="mt-1 text-xs text-slate-700">
											{formatDate(snapshot.createdAt)}
										</p>
									</div>

									<div className="flex items-center gap-2">
										<button
											type="button"
											className={ui.ghostBtn}
											disabled={busy}
											onClick={() => onRequestRestore(snapshot)}
										>
											<span>
												<RestoreIcon aria-label="Restore snapshot" />
												<span className="sr-only">
													Restore snapshot created at {formatDate(snapshot.createdAt)}
												</span>
											</span>
										</button>

										<button
											type="button"
											className={ui.ghostBtn}
											disabled={busy}
											onClick={() => onRequestDelete(snapshot)}
										>
											<span>
												<DeleteIcon aria-label="Delete snapshot" />
												<span className="sr-only">
													Delete snapshot created at {formatDate(snapshot.createdAt)}
												</span>
											</span>
										</button>
									</div>
								</div>

								<dl className="mt-3 grid gap-1 text-xs text-slate-800">
									<div className="flex flex-wrap gap-1">
										<dt className="font-semibold">Files:</dt>
										<dd>{snapshot.fileCount}</dd>
									</div>
									<div className="flex flex-wrap gap-1">
										<dt className="font-semibold">Size:</dt>
										<dd>{formatBytes(snapshot.bytesTotal)}</dd>
									</div>
									<div className="flex flex-wrap gap-1">
										<dt className="font-semibold">Metadata keys:</dt>
										<dd>{snapshot.metadataCount}</dd>
									</div>
									<div>
										<dt className="font-semibold">Storage:</dt>
										<dd className="mt-0.5 font-mono break-all">{snapshot.storagePath}</dd>
									</div>
								</dl>
							</article>
						))}
					</div>
				)}
			</aside>
		</>
	);
}