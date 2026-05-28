import { formatDate, shortId } from "../../lib/format";
import type { BondListItem, BondStatus } from "../../lib/types";
import { cx, ui } from "../../lib/ui";
import { DeleteIcon, EditIcon } from "../ui/Icons";

interface BondCardProps {
	bond: BondListItem;
	onEdit?: (bond: BondListItem) => void;
	onDelete?: (bond: BondListItem) => void;
	busy?: boolean;
}

// Keep all status classes static for Tailwind class detection.
const statusTone: Record<BondStatus, string> = {
	healthy: "border-green-700 bg-green-100/25 text-green-900",
	warning: "border-yellow-700 bg-yellow-200/20 text-yellow-900",
	broken: "border-red-800 bg-red-300/15 text-red-950",
};

export function BondCard({ bond, onEdit, onDelete, busy = false }: BondCardProps) {
	return (
		<article className={ui.card}>
			<div className="flex items-start justify-between gap-3 relative">
				<div className="flex-1 flex flex-col gap-1 xl:flex-row xl:items-center lg:gap-2 relative h-fit">
					<h3 className="text-base font-semibold text-slate-700/80 tracking-tight">
						{bond.name ?? shortId(bond.id)}
					</h3>
					<p className="mt-1 text-[11px] uppercase tracking-[0.08em] text-slate-950/70 font-mono font-extrabold">
						{shortId(bond.id)}
					</p>
				</div>

				<div className="flex flex-col items-end gap-1 absolute h-auto right-0">
					<span className={cx(ui.statusPillBase, statusTone[bond.status], "flex-1")}>{bond.status}</span>
					{/* Card actions are optional so existing call sites compile before dialog wiring is added. */}
					<div className="mt-2 flex items-center gap-2 xl:flex-col xl:absolute xl:bottom-full xl:right-0 xl:translate-y-12 h-1/12 xl:items-end xl:justify-start">
						<button
							type="button"
							className={ui.ghostBtn}
							disabled={busy || !onEdit}
							onClick={() => onEdit?.(bond)}
							aria-label={`Edit bond ${bond.name ?? shortId(bond.id)}`}
						>
							<EditIcon className="fill-zinc-700" />
							<span>Edit</span>
						</button>

						<button
							type="button"
							className={ui.ghostBtn}
							disabled={busy || !onDelete}
							onClick={() => onDelete?.(bond)}
							aria-label={`Delete bond ${bond.name ?? shortId(bond.id)}`}
						>
							<DeleteIcon className="fill-zinc-700" />
							<span>Delete</span>
						</button>
					</div>
				</div>
			</div>

			<dl className="mt-3 grid gap-2">
				<div className="xl:w-7/12">
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold max-w-20">Source</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-900/90 font-mono bg-blend-saturation backdrop:from-slate-200/35 backdrop:to-slate-300/35 w-fit rounded">{bond.source}</dd>
				</div>
				<div className="xl:w-3/5">
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold max-w-20">Target</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-900/90 font-mono bg-blend-saturation backdrop:from-slate-200/35 backdrop:to-slate-300/35 w-fit rounded">{bond.target}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold max-w-20">Created</dt>
					<dd className="mt-0.5 text-sm text-slate-900/90 font-mono bg-blend-saturation backdrop:from-slate-200/35 backdrop:to-slate-300/35 w-fit rounded">{formatDate(bond.createdAt)}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold max-w-20">Metadata</dt>
					<dd className="mt-0.5 text-sm text-slate-900/90 font-mono bg-blend-saturation backdrop:from-slate-200/35 backdrop:to-slate-300/35 w-fit rounded">{bond.metadataCount}</dd>
				</div>
			</dl>
		</article>
	);
}