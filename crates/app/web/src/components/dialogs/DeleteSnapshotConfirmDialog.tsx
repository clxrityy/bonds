import { formatDate, shortId } from "../../lib/format";
import type { BondListItem, SnapshotItem } from "../../lib/types";
import { cx, ui } from "../../lib/ui";
import { CancelIcon, DeleteIcon } from "../ui/Icons";

interface DeleteSnapshotConfirmDialogProps {
	open: boolean;
	bond: BondListItem | null;
	snapshot: SnapshotItem | null;
	deleting: boolean;
	deleteError: string | null;
	onCancel: () => void;
	onConfirm: () => Promise<void>;
}

export function DeleteSnapshotConfirmDialog({
	open,
	bond,
	snapshot,
	deleting,
	deleteError,
	onCancel,
	onConfirm,
}: DeleteSnapshotConfirmDialogProps) {
	if (!open || !bond || !snapshot) return null;

	return (
		<div className="fixed inset-0 z-60 flex items-center justify-center bg-black/55 p-4">
			<section className={cx(ui.panelSurface, "w-full max-w-xl p-4")}>
				<h3 className="text-base font-semibold uppercase tracking-wide text-zinc-900">
					Confirm snapshot deletion
				</h3>

				<p className="mt-2 text-sm text-slate-700">
					You are deleting snapshot <strong>{shortId(snapshot.id)}</strong> from{" "}
					<strong>{formatDate(snapshot.createdAt)}</strong>.
				</p>

				<p className="mt-2 text-xs text-slate-700">
					Bond: {bond.name ?? shortId(bond.id)} ({shortId(bond.id)})
				</p>

				<p className="mt-3 text-xs text-slate-600">
					This removes the snapshot record and on-disk snapshot data. This action cannot be undone.
				</p>

				{deleteError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mt-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">Delete failed.</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{deleteError}
						</pre>
					</div>
				) : null}

				<div className="mt-4 flex items-center gap-3">
					<button
						type="button"
						className={ui.ghostBtn}
						disabled={deleting}
						onClick={onCancel}
					>
						<span>
							<CancelIcon aria-label="Cancel delete" />
							<span className="sr-only">Cancel delete</span>
						</span>
					</button>

					<button
						type="button"
						className={ui.primaryBtn}
						disabled={deleting}
						onClick={() => void onConfirm()}
					>
						{deleting ? "Deleting…" : (
							<span>
								<DeleteIcon aria-label="Delete snapshot" className="fill-slate-300" />
								<span className="sr-only">Delete snapshot</span>
							</span>
						)}
					</button>
				</div>
			</section>
		</div>
	);
}