import { useEffect, useMemo, useState } from "react";
import type { BondDetailItem, BondMetadata } from "../../lib/types";
import { cx, ui } from "../../lib/ui";
import { PathPickerField } from "../bonds/PathPickerField";
import { pickDirectory, pickFile } from "../../lib/filePicker";

interface EditBondSaveInput {
	id: string;
	source: string;
	target: string;
	name: string | null;
	metadata: BondMetadata | null;
}

interface EditBondDialogProps {
	open: boolean;
	detail: BondDetailItem | null;
	loading: boolean;
	loadError: string | null;
	saving: boolean;
	saveError: string | null;
	onClose: () => void;
	onSave: (input: EditBondSaveInput) => Promise<void>;
}

interface MetadataRow {
	id: string;
	key: string;
	value: string;
}

function metadataToRows(metadata: BondMetadata | null): MetadataRow[] {
	if (!metadata) return [];
	return Object.entries(metadata).map(([key, value], index) => ({
		id: `${key}-${index}`,
		key,
		value,
	}));
}

export function EditBondDialog({
	open,
	detail,
	loading,
	loadError,
	saving,
	saveError,
	onClose,
	onSave,
}: EditBondDialogProps) {
	const [source, setSource] = useState("");
	const [target, setTarget] = useState("");
	const [name, setName] = useState("");
	const [rows, setRows] = useState<MetadataRow[]>([]);
	const [localError, setLocalError] = useState<string | null>(null);
	const [picking, setPicking] = useState(false);

	// Re-seed form state every time a new detail payload is loaded.
	useEffect(() => {
		if (!open || !detail) return;
		setSource(detail.source);
		setTarget(detail.target);
		setName(detail.name ?? "");
		setRows(metadataToRows(detail.metadata));
		setLocalError(null);
	}, [open, detail]);

	const busy = saving || picking;

	const sortedRows = useMemo(
		() => [...rows].sort((a, b) => a.key.localeCompare(b.key)),
		[rows],
	);

	async function pickSourceFolder() {
		setPicking(true);
		try {
			const selected = await pickDirectory("Select source folder");
			if (selected) setSource(selected);
		} finally {
			setPicking(false);
		}
	}

	async function pickSourceFile() {
		setPicking(true);
		try {
			const selected = await pickFile("Select source file");
			if (selected) setSource(selected);
		} finally {
			setPicking(false);
		}
	}

	async function pickTargetFolder() {
		setPicking(true);
		try {
			const selected = await pickDirectory("Select target folder");
			if (selected) setTarget(selected);
		} finally {
			setPicking(false);
		}
	}

	function addMetadataRow() {
		setRows((prev) => [
			...prev,
			{ id: `new-${Date.now()}-${prev.length}`, key: "", value: "" },
		]);
	}

	function removeMetadataRow(id: string) {
		setRows((prev) => prev.filter((row) => row.id !== id));
	}

	function updateMetadataRow(id: string, patch: Partial<MetadataRow>) {
		setRows((prev) =>
			prev.map((row) => (row.id === id ? { ...row, ...patch } : row)),
		);
	}

	function validateAndBuildMetadata(): BondMetadata | null {
		const next: BondMetadata = {};

		for (const row of rows) {
			const key = row.key.trim();
			const value = row.value.trim();

			// Allow fully empty rows so users can add/remove without friction.
			if (!key && !value) continue;

			if (!key) {
				throw new Error("Metadata keys cannot be blank.");
			}

			if (Object.prototype.hasOwnProperty.call(next, key)) {
				throw new Error(`Duplicate metadata key: "${key}".`);
			}

			next[key] = value;
		}

		return Object.keys(next).length > 0 ? next : null;
	}

	async function submit(event: React.FormEvent<HTMLFormElement>) {
		event.preventDefault();
		setLocalError(null);

		if (!detail) return;

		const nextSource = source.trim();
		const nextTarget = target.trim();
		const nextName = name.trim();

		if (!nextSource) {
			setLocalError("Source cannot be empty.");
			return;
		}
		if (!nextTarget) {
			setLocalError("Target cannot be empty.");
			return;
		}

		try {
			const metadata = validateAndBuildMetadata();
			await onSave({
				id: detail.id,
				source: nextSource,
				target: nextTarget,
				name: nextName ? nextName : null,
				metadata,
			});
		} catch (error) {
			setLocalError(error instanceof Error ? error.message : String(error));
		}
	}

	if (!open) return null;

	return (
		<div className="fixed inset-0 z-50 flex items-center justify-center bg-black/45 p-4">
			<section className={cx(ui.panelSurface, "w-full max-w-4xl p-4")}>
				<div className="mb-3 flex items-center justify-between">
					<h3 className="text-base font-semibold uppercase tracking-wide text-zinc-900">
						Edit bond
					</h3>
					<button
						type="button"
						className={ui.ghostBtn}
						disabled={busy}
						onClick={onClose}
					>
						Close
					</button>
				</div>

				{loading ? <div className={ui.stateCard}>Loading bond detail…</div> : null}

				{loadError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mb-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">
							Failed to load bond detail.
						</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{loadError}
						</pre>
					</div>
				) : null}

				{localError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mb-3")}>
						<p className="text-xs text-rose-700">{localError}</p>
					</div>
				) : null}

				{saveError ? (
					<div className={cx(ui.stateCard, ui.stateCardError, "mb-3")}>
						<p className="mb-1 text-xs font-medium text-rose-700">
							Failed to save changes.
						</p>
						<pre className="overflow-x-auto whitespace-pre-wrap text-xs text-slate-700">
							{saveError}
						</pre>
					</div>
				) : null}

				{detail ? (
					<form className="grid gap-4" onSubmit={submit}>
						<div className="grid gap-3 lg:grid-cols-3">
							<PathPickerField
								id="edit-source"
								label="Source"
								value={source}
								required
								disabled={busy}
								onChange={setSource}
								placeholder="e.g. /Users/me/projects/dotfiles"
								actions={[
									{ label: "Folder", onClick: pickSourceFolder },
									{ label: "File", onClick: pickSourceFile },
								]}
							/>

							<PathPickerField
								id="edit-target"
								label="Target"
								value={target}
								required
								disabled={busy}
								onChange={setTarget}
								placeholder="e.g. /Users/me/Bonds/dotfiles"
								actions={[{ label: "Folder", onClick: pickTargetFolder }]}
							/>

							<div className="flex flex-col gap-1 w-full">
								<label htmlFor="edit-name" className="text-sm font-medium text-slate-700">
									Name (optional)
								</label>
								<input
									id="edit-name"
									name="edit-name"
									type="text"
									className={ui.formInput}
									disabled={busy}
									value={name}
									onChange={(e) => setName(e.target.value)}
									placeholder="e.g. dotfiles"
								/>
							</div>
						</div>

						<div className={cx(ui.panelSurface, "p-3")}>
							<div className="mb-2 flex items-center justify-between">
								<h4 className="text-sm font-semibold uppercase tracking-wide text-zinc-900">
									Metadata
								</h4>
								<button
									type="button"
									className={ui.ghostBtn}
									disabled={busy}
									onClick={addMetadataRow}
								>
									Add Row
								</button>
							</div>

							{sortedRows.length === 0 ? (
								<p className="text-xs text-slate-600">No metadata rows.</p>
							) : (
								<div className="grid gap-2">
									{sortedRows.map((row) => (
										<div key={row.id} className="grid gap-2 md:grid-cols-[1fr_1fr_auto]">
											<input
												type="text"
												className={ui.formInput}
												placeholder="key"
												disabled={busy}
												value={row.key}
												onChange={(e) =>
													updateMetadataRow(row.id, { key: e.target.value })
												}
											/>
											<input
												type="text"
												className={ui.formInput}
												placeholder="value"
												disabled={busy}
												value={row.value}
												onChange={(e) =>
													updateMetadataRow(row.id, { value: e.target.value })
												}
											/>
											<button
												type="button"
												className={ui.ghostBtn}
												disabled={busy}
												onClick={() => removeMetadataRow(row.id)}
											>
												Remove
											</button>
										</div>
									))}
								</div>
							)}
						</div>

						<div className="flex items-center gap-3">
							<button type="submit" className={ui.primaryBtn} disabled={busy || loading}>
								Save Changes
							</button>
							{picking ? <span className="text-xs text-slate-600">Opening picker…</span> : null}
							{saving ? <span className="text-xs text-slate-600">Saving…</span> : null}
						</div>
					</form>
				) : null}
			</section>
		</div>
	);
}