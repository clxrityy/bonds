import { useEffect, useState } from "react";
import { readJson, writeJson } from "../lib/storage";

// Generic persisted state hook used by layout and future preferences.
export function usePersistedState<T>(key: string, fallback: T) {
	const [value, setValue] = useState<T>(() => readJson<T>(key, fallback));

	useEffect(() => {
		writeJson(key, value);
	}, [key, value]);

	return [value, setValue] as const;
}