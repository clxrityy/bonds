// Presentation helpers kept pure and reusable.

export function shortId(id: string): string {
	return id.slice(0, 8);
}

export function formatDate(value: string): string {
	const date = new Date(value);
	return Number.isNaN(date.valueOf()) ? value : date.toLocaleString();
}