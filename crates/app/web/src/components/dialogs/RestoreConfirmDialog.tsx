import { formatDate, shortId } from "../../lib/format";
import type { BondListItem, SnapshotItem } from "../../lib/types";
import { cx, ui } from "../../lib/ui";
import { CancelIcon, RestoreIcon } from "../ui/Icons";

interface RestoreConfirmDialogProps {
	open: boolean;
	bond: BondListItem | null;
	snapshot: SnapshotItem | null;
	restoring: boolean;
	restoreError: string | null;
	onCancel: () => void;
	onConfirm: () => Promise<void>;
}

export function RestoreConfirmDialog({
	open,
	bond,
	snapshot,
	restoring,
	restoreError,
	onCancel,
	onConfirm,
}: RestoreConfirmDialogProps) {
	if (!open || !bond || !snapshot) return null;

	return (
		<div className="fixed inset-0 z-60 flex items-center justify-center bg-black/55 p-4">
			<section className={cx(ui.panelSurface, "w-full max-w-xl p-4")}>
				<h3 className="text-base font-semibold uppercase tracking-wide text-zinc-900">
					Confirm restore
				</h3>

				<p className="mt-2 text-sm text-slate-700">
					You are restoring snapshot <strong>{shortId(snapshot.id)}</strong> from{" "}
					<strong>{formatDate(snapshot.createdAt)}</strong>.
				</p>

				<p className="mt-2 text-xs text-slate-700">
					Bond: {bond.name ?? shortId(bond.id)} ({shortId(bond.id)})
				</p>

				<p className="mt-3 text-xs text-slate-600">
					Safety behavior: if live source content exists, Bonds creates a safety snapshot
					first, then performs the restore. This gives you a rollback path for the pre-restore state.
				</p>

				{restoreError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mt-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">Restore failed.</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{restoreError}
						</pre>
					</div>
				) : null}

				<div className="mt-4 flex items-center gap-3">
					<button
						type="button"
						className={ui.ghostBtn}
						disabled={restoring}
						onClick={onCancel}
					>
						<span>
							<CancelIcon aria-label="Cancel restore" />
							<span className="sr-only">Cancel restore</span>
						</span>
					</button>
					<button
						type="button"
						className={ui.primaryBtn}
						disabled={restoring}
						onClick={() => void onConfirm()}
					>
						{restoring ? "Restoring…" : <span>
								<RestoreIcon aria-label="Restore snapshot" />
								<span className="sr-only">Restore snapshot</span>
							</span>}
					</button>
				</div>
			</section>
		</div>
	);
}