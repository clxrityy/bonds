import { useCallback, useEffect, useState } from "react";
import {
	createBond,
	deleteBond,
	getBondDetail,
	listBonds,
	updateBond,
	updateBondMetadata,
} from "../lib/tauri";
import type {
	BondDetailItem,
	BondListItem,
	CreateBondInput,
	DeleteBondInput,
	UpdateBondInput,
	UpdateBondMetadataInput,
} from "../lib/types";

export function useBonds() {
	const [bonds, setBonds] = useState<BondListItem[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [error, setError] = useState<string | null>(null);

	const [creating, setCreating] = useState<boolean>(false);
	const [createError, setCreateError] = useState<string | null>(null);

	const [updating, setUpdating] = useState<boolean>(false);
	const [updateError, setUpdateError] = useState<string | null>(null);

	const [deleting, setDeleting] = useState<boolean>(false);
	const [deleteError, setDeleteError] = useState<string | null>(null);

	const refresh = useCallback(async () => {
		setLoading(true);
		setError(null);

		try {
			const next = await listBonds();
			setBonds(next);
		} catch (err) {
			setError(err instanceof Error ? err.message : String(err));
		} finally {
			setLoading(false);
		}
	}, []);

	const getDetail = useCallback(async (id: string): Promise<BondDetailItem> => {
		return getBondDetail(id);
	}, []);

	const create = useCallback(
		async (input: CreateBondInput) => {
			setCreating(true);
			setCreateError(null);

			try {
				const created = await createBond(input);
				// Keep backend as source of truth after mutation.
				await refresh();
				return created;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setCreateError(message);
				throw err;
			} finally {
				setCreating(false);
			}
		},
		[refresh],
	);

	const update = useCallback(
		async (input: UpdateBondInput) => {
			setUpdating(true);
			setUpdateError(null);

			try {
				const updated = await updateBond(input);
				await refresh();
				return updated;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setUpdateError(message);
				throw err;
			} finally {
				setUpdating(false);
			}
		},
		[refresh],
	);

	const saveMetadata = useCallback(
		async (input: UpdateBondMetadataInput) => {
			setUpdating(true);
			setUpdateError(null);

			try {
				const updated = await updateBondMetadata(input);
				await refresh();
				return updated;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setUpdateError(message);
				throw err;
			} finally {
				setUpdating(false);
			}
		},
		[refresh],
	);

	const remove = useCallback(
		async (input: DeleteBondInput) => {
			setDeleting(true);
			setDeleteError(null);

			try {
				const deleted = await deleteBond(input);
				await refresh();
				return deleted;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setDeleteError(message);
				throw err;
			} finally {
				setDeleting(false);
			}
		},
		[refresh],
	);

	useEffect(() => {
		void refresh();
	}, [refresh]);

	return {
		bonds,
		loading,
		error,
		refresh,

		creating,
		createError,
		create,

		updating,
		updateError,
		update,
		saveMetadata,

		deleting,
		deleteError,
		remove,

		getDetail,
	};
}