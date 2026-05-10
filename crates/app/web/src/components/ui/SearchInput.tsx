import { ui } from "../../lib/ui";

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
			className={ui.searchInput}
			type="search"
			value={value}
			placeholder={placeholder}
			onChange={(e) => onChange(e.target.value)}
		/>
	);
}