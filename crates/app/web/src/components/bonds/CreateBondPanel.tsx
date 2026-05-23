import type { BondListItem, CreateBondInput } from "../../lib/types";
import { ui, cx } from "../../lib/ui";
import { AddIcon } from "../ui/Icons";

interface CreateBondPanelProps {
	onCreate: (input: CreateBondInput) => Promise<BondListItem>;
	creating: boolean;
	createError: string | null;
}

export function CreateBondPanel({
	onCreate,
	creating,
	createError,
}: CreateBondPanelProps) {
	return (
		<div className="border-t pt-4">
			{/* Keeping existing behavior for now:
                while creating or when there is an error, this status panel is shown. */}
			{creating || createError ? (
				<div className={cx("p-4 rounded-md", createError ? ui.stateCardError : ui.stateCard)}>
					{createError ? (
						<>
							<p className="mb-2 font-medium text-rose-500">Failed to create bond.</p>
							<pre className="overflow-x-auto whitespace-pre-wrap wrap-break-word text-xs text-slate-700">
								{createError}
							</pre>
						</>
					) : (
						<p className="text-slate-700">Creating bond…</p>
					)}
				</div>
			) : (
				<div>
					<h3 className="text-lg font-medium text-slate-900/75 mb-2">Create new bond</h3>
					<p className="text-sm text-slate-700 mb-3">
						Enter a source and an optional target to create a new bond.
					</p>

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
								// We’ll replace this alert with proper inline validation in the next step.
								alert("Source is required.");
								return;
							}

							try {
								await onCreate({
									source,
									target: target || null,
									name: name || null,
								});
								form.reset();
							} catch (err) {
								// Error UI is shown by createError state from the parent hook.
								console.error("Failed to create bond:", err);
							}
						}}
					>
						<div className="flex flex-col sm:flex-row items-start sm:items-end gap-3 w-full">
							<div className="flex flex-col gap-1 w-full">
								<label htmlFor="source" className="text-sm font-medium text-slate-700">
									Source
								</label>
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
								<label htmlFor="target" className="text-sm font-medium text-slate-700">
									Target (optional)
								</label>
								<input
									type="text"
									name="target"
									id="target"
									className={ui.formInput}
									placeholder="e.g. /path/to/related-file.txt or another-entity-id"
								/>
							</div>

							<div className="flex flex-col gap-1 w-full">
								<label htmlFor="name" className="text-sm font-medium text-slate-700">
									Name (optional)
								</label>
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
							<AddIcon className="fill-zinc-200" />
							<span>Create Bond</span>
						</button>
					</form>
				</div>
			)}
		</div>
	);
}