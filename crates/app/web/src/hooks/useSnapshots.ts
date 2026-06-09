import { useCallback, useEffect, useState } from "react";
import {
	createBondSnapshot,
	deleteBondSnapshot,
	listBondSnapshots,
	restoreBondSnapshot,
} from "../lib/tauri";
import type { RestoreSnapshotResult, SnapshotItem } from "../lib/types";

interface UseSnapshotsOptions {
	bondId: string | null;
	enabled?: boolean;
}

export function useSnapshots({ bondId, enabled = true }: UseSnapshotsOptions) {
	const [snapshots, setSnapshots] = useState<SnapshotItem[]>([]);
	const [loading, setLoading] = useState<boolean>(false);
	const [error, setError] = useState<string | null>(null);

	const [creating, setCreating] = useState<boolean>(false);
	const [restoring, setRestoring] = useState<boolean>(false);
	const [deleting, setDeleting] = useState<boolean>(false);
	const [actionError, setActionError] = useState<string | null>(null);

	const refresh = useCallback(async () => {
		if (!enabled || !bondId) {
			setSnapshots([]);
			setError(null);
			setLoading(false);
			return;
		}

		setLoading(true);
		setError(null);

		try {
			const next = await listBondSnapshots(bondId);
			setSnapshots(next);
		} catch (err) {
			setError(err instanceof Error ? err.message : String(err));
		} finally {
			setLoading(false);
		}
	}, [bondId, enabled]);

	const createNow = useCallback(async (): Promise<SnapshotItem | null> => {
		if (!bondId) return null;

		setCreating(true);
		setActionError(null);

		try {
			const created = await createBondSnapshot(bondId);
			await refresh();
			return created;
		} catch (err) {
			const message = err instanceof Error ? err.message : String(err);
			setActionError(message);
			throw err;
		} finally {
			setCreating(false);
		}
	}, [bondId, refresh]);

	const restore = useCallback(
		async (snapshotId: string): Promise<RestoreSnapshotResult | null> => {
			if (!bondId) return null;

			setRestoring(true);
			setActionError(null);

			try {
				const result = await restoreBondSnapshot({ id: bondId, snapshotId });
				await refresh();
				return result;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setActionError(message);
				throw err;
			} finally {
				setRestoring(false);
			}
		},
		[bondId, refresh],
	);

	const removeSnapshot = useCallback(
		async (snapshotId: string): Promise<SnapshotItem | null> => {
			if (!bondId) return null;

			setDeleting(true);
			setActionError(null);

			try {
				const deleted = await deleteBondSnapshot({ id: bondId, snapshotId });
				await refresh();
				return deleted;
			} catch (err) {
				const message = err instanceof Error ? err.message : String(err);
				setActionError(message);
				throw err;
			} finally {
				setDeleting(false);
			}
		},
		[bondId, refresh],
	);

	const clearActionError = useCallback(() => {
		setActionError(null);
	}, []);

	useEffect(() => {
		void refresh();
	}, [refresh]);

	return {
		snapshots,
		loading,
		error,
		refresh,

		creating,
		restoring,
		deleting,
		actionError,
		clearActionError,

		createNow,
		restore,
		removeSnapshot,
	};
}