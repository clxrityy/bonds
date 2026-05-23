import { useCallback, useState } from "react";
import type { FormEvent } from "react";
import type { BondListItem, CreateBondInput } from "../lib/types";
import { pickDirectory, pickFile } from "../lib/filePicker";
import { suggestTargetFromParent } from "../lib/bondPaths";

interface UseCreateBondFormArgs {
	onCreate: (input: CreateBondInput) => Promise<BondListItem>;
}

interface FormValues {
	source: string;
	target: string;
	name: string;
}

const INITIAL_VALUES: FormValues = {
	source: "",
	target: "",
	name: "",
};

export function useCreateBondForm({ onCreate }: UseCreateBondFormArgs) {
	const [values, setValues] = useState<FormValues>(INITIAL_VALUES);
	const [localError, setLocalError] = useState<string | null>(null);
	const [picking, setPicking] = useState<boolean>(false);

	const setSource = useCallback((source: string) => {
		setValues((prev) => ({ ...prev, source }));
		setLocalError(null);
	}, []);

	const setTarget = useCallback((target: string) => {
		setValues((prev) => ({ ...prev, target }));
		setLocalError(null);
	}, []);

	const setName = useCallback((name: string) => {
		setValues((prev) => ({ ...prev, name }));
		setLocalError(null);
	}, []);

	const reset = useCallback(() => {
		setValues(INITIAL_VALUES);
		setLocalError(null);
	}, []);

	const pickSourceFolder = useCallback(async () => {
		setPicking(true);
		try {
			const selected = await pickDirectory("Select source folder");
			if (selected) setSource(selected);
		} finally {
			setPicking(false);
		}
	}, [setSource]);

	const pickSourceFile = useCallback(async () => {
		setPicking(true);
		try {
			const selected = await pickFile("Select source file");
			if (selected) setSource(selected);
		} finally {
			setPicking(false);
		}
	}, [setSource]);

	const pickTargetFolder = useCallback(async () => {
		setPicking(true);
		try {
			const parent = await pickDirectory("Select target parent folder");
			if (!parent) return;
			const suggested = await suggestTargetFromParent(parent, values.source);
			setTarget(suggested);
		} finally {
			setPicking(false);
		}
	}, [values.source, setTarget]);

	const submit = useCallback(
		async (event: FormEvent<HTMLFormElement>) => {
			event.preventDefault();

			const source = values.source.trim();
			const target = values.target.trim();
			const name = values.name.trim();

			if (!source) {
				setLocalError("Source is required.");
				return;
			}

			await onCreate({
				source,
				target: target || null,
				name: name || null,
			});

			reset();
		},
		[values, onCreate, reset],
	);

	return {
		values,
		localError,
		picking,
		setSource,
		setTarget,
		setName,
		pickSourceFolder,
		pickSourceFile,
		pickTargetFolder,
		submit,
	};
}