interface ResizeHandleProps {
	onPointerDown: (event: React.PointerEvent<HTMLDivElement>) => void;
}

export function ResizeHandle({ onPointerDown }: ResizeHandleProps) {
	return (
		<div
			className="absolute -right-1 top-0 h-full w-2 cursor-ew-resize bg-transparent hover:bg-sky-400/25"
			role="separator"
			aria-orientation="vertical"
			aria-label="Resize side panel"
			onPointerDown={onPointerDown}
		/>
	);
}