// Tiny safe JSON storage helpers to avoid repetitive try/catch in hooks.

export function readJson<T>(key: string, fallback: T): T {
	if (typeof window === "undefined") return fallback;

	try {
		const raw = window.localStorage.getItem(key);
		if (!raw) return fallback;
		return JSON.parse(raw) as T;
	} catch {
		// If parsing fails, fallback keeps the app bootable.
		return fallback;
	}
}

export function writeJson<T>(key: string, value: T): void {
	if (typeof window === "undefined") return;

	try {
		window.localStorage.setItem(key, JSON.stringify(value));
	} catch {
		// Best-effort persistence: don't break UI if storage is unavailable.
	}
}