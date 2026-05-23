import { join } from "@tauri-apps/api/path";

export function pathLeaf(input: string): string {
	// Normalize trailing separators, then take last segment.
	const normalized = input.trim().replace(/[\\/]+$/g, "");
	if (!normalized) return "";
	const parts = normalized.split(/[\\/]/);
	return parts[parts.length - 1] ?? "";
}

export async function suggestTargetFromParent(
	parentDir: string,
	sourcePath: string,
): Promise<string> {
	const leaf = pathLeaf(sourcePath);
	if (!leaf) return parentDir;
	return join(parentDir, leaf);
}