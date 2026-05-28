import type { BondListItem } from "../../lib/types";
import { cx, ui } from "../../lib/ui";

interface DeleteBondDialogProps {
	open: boolean;
	bond: BondListItem | null;
	deleting: boolean;
	deleteError: string | null;
	onCancel: () => void;
	onConfirm: () => Promise<void>;
}

export function DeleteBondDialog({
	open,
	bond,
	deleting,
	deleteError,
	onCancel,
	onConfirm,
}: DeleteBondDialogProps) {
	if (!open || !bond) return null;

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/45 p-4">
			<section className={cx(ui.panelSurface, "w-full max-w-xl p-4")}>
				<h3 className="text-base font-semibold uppercase tracking-wide text-zinc-900">
					Delete bond
				</h3>

				<p className="mt-2 text-sm text-slate-700">
					You are deleting:
				</p>
				<p className="mt-1 text-xs font-mono text-slate-700 break-all">
					{bond.name ?? bond.id}
				</p>

				<p className="mt-3 text-xs text-slate-600">
					Safe mode is enabled. The app will not force-delete real files/folders at the target path.
				</p>

				{bond.status === "warning" ? (
					<p className="mt-2 text-xs text-amber-700">
						This bond is in warning state. If the target is no longer a symlink, safe delete may be blocked.
					</p>
				) : null}

				{deleteError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mt-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">
							Failed to delete bond.
						</p>
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
						Cancel
					</button>
					<button
						type="button"
						className={ui.primaryBtn}
						disabled={deleting}
						onClick={() => void onConfirm()}
					>
						{deleting ? "Deleting…" : "Delete Bond"}
					</button>
				</div>
			</section>
		</div>
	);
}