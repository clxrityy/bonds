import { useCallback, useRef } from "react";
import { clampSideWidth } from "../lib/layout";

interface Args {
	enabled: boolean;
	currentWidth: number;
	onWidthChange: (nextWidth: number) => void;
};

// Pointer-drag logic isolated so panel component stays presentational.
export function useResizableSidePanel({ enabled, currentWidth, onWidthChange }: Args) {
	const startXRef = useRef(0);
	const startWidthRef = useRef(0);

	const onPointerDown = useCallback(
		(event: React.PointerEvent<HTMLDivElement>) => {
			if (!enabled) return;

			event.preventDefault();
			startXRef.current = event.clientX;
			startWidthRef.current = currentWidth;

			const onMove = (moveEvent: PointerEvent) => {
				const delta = moveEvent.clientX - startXRef.current;
				onWidthChange(clampSideWidth(startWidthRef.current + delta));
			};

			const onUp = () => {
				window.removeEventListener("pointermove", onMove);
				window.removeEventListener("pointerup", onUp);
			};

			window.addEventListener("pointermove", onMove);
			window.addEventListener("pointerup", onUp);
		},
		[enabled, currentWidth, onWidthChange]
	);

	return { onPointerDown };
}