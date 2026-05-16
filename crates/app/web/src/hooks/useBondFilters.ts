import { useMemo } from "react";
import type { BondListItem, ViewTab } from "../lib/types";

type TabCounts = Record<ViewTab, number>;

export function useBondFilters(bonds: BondListItem[], search: string, activeTab: ViewTab) {
	const counts = useMemo<TabCounts>(() => {
		const next: TabCounts = {
			all: bonds.length,
			healthy: 0,
			warning: 0,
			broken: 0,
		};

		for (const bond of bonds) {
			next[bond.status] += 1;
		}

		return next;
	}, [bonds]);

	const filtered = useMemo(() => {
		const q = search.trim().toLowerCase();

		return bonds.filter((bond) => {
			if (activeTab !== "all" && bond.status !== activeTab) {
				return false;
			}

			if (!q) return true;

			const haystack = [
				bond.name ?? "",
				bond.source,
				bond.target,
				bond.status,
			]
				.join(" ")
				.toLowerCase();

			return haystack.includes(q);
		});
	}, [bonds, search, activeTab]);

	return { filtered, counts };
}