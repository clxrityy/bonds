import type { BondListItem } from "../../lib/types";
import { BondCard } from "./BondCard";

interface BondViewerProps {
	loading: boolean;
	error: string | null;
	bonds: BondListItem[];
	onRefresh: () => void;
}

export function BondViewer({ loading, error, bonds, onRefresh }: BondViewerProps) {
	return (
		<section className="grid gap-4">
			<div className="flex flex-wrap items-center justify-between gap-3">
				<h2 className="text-xl font-semibold text-slate-100">Current bonds</h2>
				<button
					type="button"
					className="rounded-lg border border-slate-700 bg-slate-800 px-3 py-2 text-sm font-medium text-slate-100 transition-colors hover:bg-slate-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70"
					onClick={onRefresh}
				>
					Refresh
				</button>
			</div>

			{loading ? (
				<div className="rounded-xl border border-dashed border-slate-700 bg-slate-900 p-4 text-slate-200">
					Loading bonds…
				</div>
			) : error ? (
				<div className="rounded-xl border border-dashed border-rose-500/50 bg-slate-900 p-4 text-slate-200">
					<p className="mb-2 font-medium text-rose-300">Unable to load bonds.</p>
					<pre className="overflow-x-auto whitespace-pre-wrap break-words text-xs text-slate-300">
						{error}
					</pre>
				</div>
			) : bonds.length === 0 ? (
				<div className="rounded-xl border border-dashed border-slate-700 bg-slate-900 p-4 text-slate-300">
					No bonds match this view.
				</div>
			) : (
				<div className="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-3">
					{bonds.map((bond) => (
						<BondCard key={bond.id} bond={bond} />
					))}
				</div>
			)}
		</section>
	);
}