import { useCallback, useEffect, useState } from "react";
import { createBond, listBonds } from "../lib/tauri";
import type { BondListItem, CreateBondInput } from "../lib/types";

export function useBonds() {
	const [bonds, setBonds] = useState<BondListItem[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [error, setError] = useState<string | null>(null);

	const [creating, setCreating] = useState<boolean>(false);
	const [createError, setCreateError] = useState<string | null>(null);

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

	const create = useCallback(async (input: CreateBondInput) => {
		setCreating(true);
		setCreateError(null);

		try {
			const created = await createBond(input);
			// Keep it simple and consistent with server truth.
			await refresh();
			return created;
		} catch (err) {
			const message = err instanceof Error ? err.message : String(err);
			setCreateError(message);
			throw err;
		} finally {
			setCreating(false);
		}
	}, [refresh]);

	useEffect(() => {
		void refresh();
	}, [refresh]);

	return { bonds, loading, error, refresh, creating, createError, create };
}