import { useCallback, useEffect, useState } from "react";
import { listBonds } from "../lib/tauri";
import type { BondListItem } from "../lib/types";

export function useBonds() {
	const [bonds, setBonds] = useState<BondListItem[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [error, setError] = useState<string | null>(null);

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

	useEffect(() => {
		void refresh();
	}, [refresh]);

	return { bonds, loading, error, refresh };
}