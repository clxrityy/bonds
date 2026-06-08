import type { BondListItem, CreateBondInput } from "../../lib/types";
import { ui, cx } from "../../lib/ui";
import { RefreshIcon } from "../ui/Icons";
import { BondCard } from "./BondCard";
import { CreateBondPanel } from "./CreateBondPanel";

interface BondViewerProps {
	loading: boolean;
	error: string | null;
	bonds: BondListItem[];
	onRefresh: () => void;
	onCreate: (input: CreateBondInput) => Promise<BondListItem>;
	creating: boolean;
	createError: string | null;

	onEditBond?: (bond: BondListItem) => void;
	onDeleteBond?: (bond: BondListItem) => void;
	onHistoryBond?: (bond: BondListItem) => void;
	actionBusy?: boolean;
}

export function BondViewer({
	loading,
	error,
	bonds,
	onRefresh,
	onCreate,
	creating,
	createError,
	onEditBond,
	onDeleteBond,
	onHistoryBond,
	actionBusy = false,
}: BondViewerProps) {
	return (
		<section className="grid gap-4">
			<div className="flex flex-wrap items-center justify-between gap-3">
				<h2 className="text-xl lg:text-2xl text-slate-900/75 uppercase heading-text-shadow font-science-gothic font-semibold text-shadow-2xs">
					Current bonds
				</h2>
				<button type="button" className={ui.primaryBtn} onClick={onRefresh}>
					<RefreshIcon className="fill-zinc-300" />
					<span>Refresh</span>
					<span className="sr-only">Refresh bond data</span>
				</button>
			</div>

			<CreateBondPanel onCreate={onCreate} creating={creating} createError={createError} />

			{loading ? (
				<div className={ui.stateCard}>Loading bonds…</div>
			) : error ? (
				<div className={cx(ui.stateCard, ui.stateCardError)}>
					<p className="mb-2 font-medium text-rose-500">Unable to load bonds.</p>
					<pre className="overflow-x-auto whitespace-pre-wrap wrap-break-word text-xs text-slate-700">
						{error}
					</pre>
				</div>
			) : bonds.length === 0 ? (
				<div className={ui.stateCard}>
					<p className="text-slate-700">No bonds match this view.</p>
				</div>
			) : (
				<div className="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-3">
					{bonds.map((bond) => (
						<BondCard
							key={bond.id}
							bond={bond}
							onHistory={onHistoryBond}
							onEdit={onEditBond}
							onDelete={onDeleteBond}
							busy={actionBusy}
						/>
					))}
				</div>
			)}
		</section>
	);
}