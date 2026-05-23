import { ui } from "../../lib/ui";

export interface PathPickerAction {
	label: string;
	onClick: () => void | Promise<void>;
	disabled?: boolean;
}

interface PathPickerFieldProps {
	id: string;
	label: string;
	value: string;
	onChange: (value: string) => void;
	placeholder?: string;
	required?: boolean;
	hint?: string;
	disabled?: boolean;
	actions?: PathPickerAction[];
}

export function PathPickerField({
	id,
	label,
	value,
	onChange,
	placeholder,
	required,
	hint,
	disabled,
	actions = [],
}: PathPickerFieldProps) {
	return (
		<div className="flex flex-col gap-1 w-full">
			<label htmlFor={id} className="text-sm font-medium text-slate-700">
				{label}
			</label>

			<div className="flex items-center gap-2">
				<input
					id={id}
					name={id}
					type="text"
					value={value}
					onChange={(e) => onChange(e.target.value)}
					placeholder={placeholder}
					required={required}
					disabled={disabled}
					className={ui.formInput}
				/>

				{actions.map((action) => (
					<button
						key={action.label}
						type="button"
						className={ui.ghostBtn}
						disabled={disabled || action.disabled}
						onClick={() => void action.onClick()}
					>
						{action.label}
					</button>
				))}
			</div>

			{hint ? <p className="text-xs text-slate-600">{hint}</p> : null}
		</div>
	);
}