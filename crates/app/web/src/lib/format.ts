// Presentation helpers kept pure and reusable.

export function shortId(id: string): string {
	return id.slice(0, 8);
}

export function formatDate(value: string): string {
	const date = new Date(value);
	return Number.isNaN(date.valueOf()) ? value : date.toLocaleString();
}

/**
 * Convert bytes to a compact human-readable unit.
 * Example: 1536 => "1.5 KB"
 */
export function formatBytes(bytes: number): string {
	if (!Number.isFinite(bytes) || bytes < 0) return String(bytes);

	const units = ["B", "KB", "MB", "GB", "TB"];
	let value = bytes;
	let unitIndex = 0;

	while (value >= 1024 && unitIndex < units.length - 1) {
		value /= 1024;
		unitIndex += 1;
	}

	const decimals = unitIndex === 0 ? 0 : 1;
	return `${value.toFixed(decimals)} ${units[unitIndex]}`;
}