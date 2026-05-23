import { open } from "@tauri-apps/plugin-dialog";

type OpenResult = string | string[] | null;

function toSinglePath(result: OpenResult): string | null {
	// Defensive conversion; we always request multiple: false.
	return typeof result === "string" ? result : null;
}

export async function pickDirectory(title: string): Promise<string | null> {
	const result = await open({
		title,
		directory: true,
		multiple: false,
	});
	return toSinglePath(result);
}

export async function pickFile(title: string): Promise<string | null> {
	const result = await open({
		title,
		directory: false,
		multiple: false,
	});
	return toSinglePath(result);
}