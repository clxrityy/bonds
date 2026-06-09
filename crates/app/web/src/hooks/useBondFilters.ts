import { useMemo } from "react";
import type { BondListItem, BondMetadata, ViewTab } from "../lib/types";

type TabCounts = Record<ViewTab, number>;

function metadataEntries(metadata: BondMetadata | null | undefined): [ string, string ][] {
	if (!metadata) return [];
	return Object.entries(metadata);
}

/**
 * Supports query tokens:
 * - meta:key            -> any metadata key containing "key"
 * - meta:key=value      -> metadata key containing "key" and value containing "value"
 * - has:metadata        -> only bonds with metadata
 * - has:no-metadata     -> only bonds without metadata
 *
 * Any other token does free-text search across name/source/target/status + metadata.
 */
function matchesMetadataToken(
	bond: BondListItem,
	token: string,
): boolean {
	const expr = token.slice("meta:".length).trim();
	const entries = metadataEntries(bond.metadata);

	if (!expr) {
		// `meta:` with no key acts as "has metadata".
		return entries.length > 0 || bond.metadataCount > 0;
	}

	const separator = expr.includes("=") ? "=" : null;

	if (!separator) {
		const keyNeedle = expr.toLowerCase();
		return entries.some(([key]) => key.toLowerCase().includes(keyNeedle));
	}

	const [rawKey, ...rawValueParts] = expr.split(separator);
	const keyNeedle = rawKey.trim().toLowerCase();
	const valueNeedle = rawValueParts.join(separator).trim().toLowerCase();

	return entries.some(([key, value]) => {
		const keyMatch = key.toLowerCase().includes(keyNeedle);
		const valueMatch = value.toLowerCase().includes(valueNeedle);
		return keyMatch && valueMatch;
	});
}

function matchesHasToken(bond: BondListItem, token: string): boolean {
	const mode = token.slice("has:".length).trim().toLowerCase();
	const count = bond.metadataCount ?? metadataEntries(bond.metadata).length;

	if (mode === "metadata") return count > 0;
	if (mode === "no-metadata" || mode === "nometadata") return count === 0;

	// Unknown `has:*` token should not exclude valid results.
	return true;
}

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
		const tokens = search
			.trim()
			.toLowerCase()
			.split(/\s+/)
			.filter(Boolean);

		return bonds.filter((bond) => {
			if (activeTab !== "all" && bond.status !== activeTab) {
				return false;
			}

			if (tokens.length === 0) return true;

			const metadata = metadataEntries(bond.metadata);
			const metadataHaystack = metadata
				.flatMap(([k, v]) => [k, v, `${k}=${v}`])
				.join(" ")
				.toLowerCase();

			const genericHaystack = [
				bond.name ?? "",
				bond.source,
				bond.target,
				bond.status,
				metadataHaystack,
			]
				.join(" ")
				.toLowerCase();

			return tokens.every((token) => {
				if (token.startsWith("meta:")) {
					return matchesMetadataToken(bond, token);
				}

				if (token.startsWith("has:")) {
					return matchesHasToken(bond, token);
				}

				return genericHaystack.includes(token);
			});
		});
	}, [bonds, search, activeTab]);

	return { filtered, counts };
}