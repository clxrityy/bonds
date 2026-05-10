interface SearchInputProps {
	value: string;
	onChange: (value: string) => void;
	placeholder?: string;
}

export function SearchInput({
	value,
	onChange,
	placeholder = "Search bonds…",
}: SearchInputProps) {
	return (
		<input
			className="w-full rounded-lg border border-slate-700 bg-slate-900 px-3 py-2 text-sm text-slate-100 placeholder:text-slate-400 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-sky-400/70"
			type="search"
			value={value}
			placeholder={placeholder}
			onChange={(e) => onChange(e.target.value)}
		/>
	);
}