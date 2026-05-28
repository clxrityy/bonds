import type { BondListItem, CreateBondInput } from "../../lib/types";
import { ui, cx } from "../../lib/ui";
import { AddIcon } from "../ui/Icons";
import { PathPickerField } from "./PathPickerField";
import { useCreateBondForm } from "../../hooks/useCreateBondForm";

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
	const form = useCreateBondForm({ onCreate });
	const busy = creating || form.picking;

	return (
		<div className="border-t pt-4">
			<h3 className="text-lg font-medium text-slate-900/75 mb-2">Create new bond</h3>
			<p className="text-sm text-slate-700/65 mb-3 text-shadow-2xs indent-4 italic">
				Type source/target paths manually, or select them from your file system.
			</p>

			{form.localError ? (
				<div className={cx(ui.stateCard, ui.stateCardError, "mb-3 p-3")}>
					<p className="text-xs text-rose-700 text-shadow-2xs">{form.localError}</p>
				</div>
			) : null}

			{createError ? (
				<div className={cx(ui.stateCard, ui.stateCardError, "mb-3 p-3")}>
					<p className="mb-1 text-xs font-medium text-rose-700 text-shadow-2xs">Failed to create bond.</p>
					<pre className="overflow-x-auto whitespace-pre-wrap wrap-break-word text-xs text-slate-700 text-shadow-2xs">
						{createError}
					</pre>
				</div>
			) : null}

			<form className="grid gap-3" onSubmit={form.submit}>
				<div className="grid gap-3 lg:grid-cols-3">
					<PathPickerField
						id="source"
						label="Source"
						value={form.values.source}
						required
						disabled={busy}
						placeholder="e.g. /Users/me/projects/dotfiles"
						hint="You can type a path or pick a folder/file."
						onChange={form.setSource}
						actions={[
							{ label: "Folder", onClick: form.pickSourceFolder },
							{ label: "File", onClick: form.pickSourceFile },
						]}
					/>

					<PathPickerField
						id="target"
						label="Target (optional)"
						value={form.values.target}
						disabled={busy}
						placeholder="e.g. /Users/me/Bonds/dotfiles"
						hint="Choose a parent folder and we’ll suggest a target path."
						onChange={form.setTarget}
						actions={[{ label: "Folder", onClick: form.pickTargetFolder }]}
					/>

					<div className="flex flex-col gap-1 w-full">
						<label htmlFor="name" className="text-sm font-medium text-slate-700">
							Name (optional)
						</label>
						<input
							id="name"
							name="name"
							type="text"
							className={ui.formInput}
							disabled={busy}
							value={form.values.name}
							onChange={(e) => form.setName(e.target.value)}
							placeholder="e.g. dotfiles"
						/>
					</div>
				</div>

				<div className="flex items-center gap-3">
					<button type="submit" className={ui.primaryBtn} disabled={busy}>
						<AddIcon className="fill-zinc-200" />
						<span>Create Bond</span>
					</button>

					{form.picking ? (
						<span className="text-xs text-slate-600">Opening file picker…</span>
					) : null}

					{creating ? (
						<span className="text-xs text-slate-600">Creating bond…</span>
					) : null}
				</div>
			</form>
		</div>
	);
}