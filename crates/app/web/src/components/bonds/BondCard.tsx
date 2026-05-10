import { formatDate, shortId } from "../../lib/format";
import type { BondListItem, BondStatus } from "../../lib/types";
import { cx, ui } from "../../lib/ui";

interface BondCardProps {
	bond: BondListItem;
}

// Keep all status classes static for Tailwind class detection.
const statusTone: Record<BondStatus, string> = {
	healthy: "border-emerald-400/40 bg-emerald-500/10 text-emerald-300",
	warning: "border-amber-400/40 bg-amber-500/10 text-amber-300",
	broken: "border-rose-400/40 bg-rose-500/10 text-rose-300",
};

export function BondCard({ bond }: BondCardProps) {
	return (
		<article className={ui.card}>
			<div className="flex items-start justify-between gap-3">
				<div>
					<h3 className="text-base font-semibold text-slate-100">
						{bond.name ?? shortId(bond.id)}
					</h3>
					<p className="mt-1 text-[11px] uppercase tracking-[0.08em] text-slate-400">
						{shortId(bond.id)}
					</p>
				</div>

				<span className={cx(ui.statusPillBase, statusTone[bond.status])}>{bond.status}</span>
			</div>

			<dl className="mt-3 grid gap-2">
				<div>
					<dt className="text-[11px] uppercase tracking-wide text-slate-400">Source</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-100">{bond.source}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wide text-slate-400">Target</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-100">{bond.target}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wide text-slate-400">Created</dt>
					<dd className="mt-0.5 text-sm text-slate-100">{formatDate(bond.createdAt)}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wide text-slate-400">Metadata</dt>
					<dd className="mt-0.5 text-sm text-slate-100">{bond.metadataCount}</dd>
				</div>
			</dl>
		</article>
	);
}