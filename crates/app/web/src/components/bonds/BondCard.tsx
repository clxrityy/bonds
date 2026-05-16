import { formatDate, shortId } from "../../lib/format";
import type { BondListItem, BondStatus } from "../../lib/types";
import { cx, ui } from "../../lib/ui";

interface BondCardProps {
	bond: BondListItem;
}

// Keep all status classes static for Tailwind class detection.
const statusTone: Record<BondStatus, string> = {
	healthy: "border-zinc-700 bg-zinc-100 text-zinc-900",
	warning: "border-stone-700 bg-stone-200 text-stone-900",
	broken: "border-neutral-800 bg-neutral-300 text-neutral-950",
};

export function BondCard({ bond }: BondCardProps) {
	return (
		<article className={ui.card}>
			<div className="flex items-start justify-between gap-3">
				<div>
					<h3 className="text-base font-semibold text-slate-700/80 tracking-tight">
						{bond.name ?? shortId(bond.id)}
					</h3>
					<p className="mt-1 text-[11px] uppercase tracking-[0.08em] text-slate-400 font-mono font-extrabold">
						{shortId(bond.id)}
					</p>
				</div>

				<span className={cx(ui.statusPillBase, statusTone[bond.status])}>{bond.status}</span>
			</div>

			<dl className="mt-3 grid gap-2">
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold">Source</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-600/90 font-mono">{bond.source}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold">Target</dt>
					<dd className="mt-0.5 wrap-break-word text-sm text-slate-600/90 font-mono">{bond.target}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold">Created</dt>
					<dd className="mt-0.5 text-sm text-slate-600/90 font-mono">{formatDate(bond.createdAt)}</dd>
				</div>
				<div>
					<dt className="text-[11px] uppercase tracking-wider text-slate-500 font-bold">Metadata</dt>
					<dd className="mt-0.5 text-sm text-slate-600/90 font-mono">{bond.metadataCount}</dd>
				</div>
			</dl>
		</article>
	);
}