import type { BondListItem, CreateBondInput } from "../../lib/types";
import { ui, cx, Icons } from "../../lib/ui";
import { BondCard } from "./BondCard";

interface BondViewerProps {
	loading: boolean;
	error: string | null;
	bonds: BondListItem[];
	onRefresh: () => void;
	onCreate: (input: CreateBondInput) => Promise<BondListItem>;
	creating: boolean;
	createError: string | null;
}

export function BondViewer({ loading, error, bonds, onRefresh, onCreate, creating, createError }: BondViewerProps) {
	return (
		<section className="grid gap-4">
			<div className="flex flex-wrap items-center justify-between gap-3">
				<h2 className="text-xl lg:text-2xl text-slate-900/75 uppercase heading-text-shadow font-science-gothic  font-semibold text-shadow-2xs">Current bonds</h2>
				<button type="button" className={ui.primaryBtn} onClick={onRefresh}>
					<Icons.Refresh />
					<span>Refresh</span>
					<span className="sr-only">Refresh bond data</span>
				</button>
			</div>


			<div className="border-t pt-4">
				<h3 className="text-lg font-medium text-slate-900/75 mb-2">Create new bond</h3>
				<p className="text-sm text-slate-700 mb-3">Enter a source and an optional target to create a new bond.</p>
				<form
					className="flex flex-col sm:flex-row items-start sm:items-end gap-3"
					onSubmit={async (e) => {
						e.preventDefault();
						const form = e.currentTarget;
						const formData = new FormData(form);
						const source = formData.get("source")?.toString().trim() ?? "";
						const target = formData.get("target")?.toString().trim() ?? "";
						const name = formData.get("name")?.toString().trim() ?? "";

						if (!source) {
							alert("Source is required.");
							return;
						}

						try {
							await onCreate({ source, target: target || null, name: name || null });
							form.reset();
						} catch (e) {
							// Error is handled by hook state, no need for extra handling here.
							console.error("Failed to create bond:", e);
						}
					}}
				>
					<div className="flex flex-col sm:flex-row items-start sm:items-end gap-3 w-full">
						<div className="flex flex-col gap-1 w-full">
							<label htmlFor="source" className="text-sm font-medium text-slate-700">Source</label>
							<input
								type="text"
								name="source"
								id="source"
								className={ui.formInput}
								placeholder="e.g. /path/to/file.txt or some-entity-id"
								required
							/>
						</div>
						<div className="flex flex-col gap-1 w-full">
							<label htmlFor="target" className="text-sm font-medium text-slate-700">Target (optional)</label>
							<input
								type="text"
								name="target"
								id="target"
								className={ui.formInput}
								placeholder="e.g. /path/to/related-file.txt or another-entity-id"
							/>
						</div>
						<div className="flex flex-col gap-1 w-full">
							<label htmlFor="name" className="text-sm font-medium text-slate-700">Name (optional)</label>
							<input
								type="text"
								name="name"
								id="name"
								className={ui.formInput}
								placeholder="e.g. 'related to', 'derived from', etc."
							/>
						</div>
					</div>
					<button type="submit" className={ui.primaryBtn} disabled={creating}>
						<Icons.Add />
						<span>Create Bond</span>
					</button>
				</form>
			</div>

			{loading ? (
				<div className={ui.stateCard}>Loading bonds…</div>
			) : error ? (
				<div className={cx(ui.stateCard, ui.stateCardError)}>
					<p className="mb-2 font-medium text-rose-300">Unable to load bonds.</p>
					<pre className="overflow-x-auto whitespace-pre-wrap wrap-break-word text-xs text-slate-700">
						{error}
					</pre>
				</div>
			) : bonds.length === 0 ? (
				<div className={ui.stateCard}>
					<p className="text-slate-700">No bonds match this view.</p>
				</div>
			) : (creating ? (
				<div className={ui.stateCard}>
					<p className="mb-2 font-medium text-slate-900/75">Creating bond…</p>
					{createError && (
						<pre className="overflow-x-auto whitespace-pre-wrap wrap-break-word text-xs text-rose-300">
							{createError}
						</pre>
					)}
				</div>
			) : (
				<div className="grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-3">
					{bonds.map((bond) => (
						<BondCard key={bond.id} bond={bond} />
					))}
				</div>
			))}
		</section>
	);
}