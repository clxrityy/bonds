import type { PanelMode } from "../../lib/types";
import type { PanelControlsProps } from "./panelTypes";
import { SidePanel } from "./SidePanel";
import { TopPanel } from "./TopPanel";

type PanelHostProps = PanelControlsProps & {
	mode: PanelMode;
	sideWidth: number;
	onSwitchToTop: () => void;
	onSwitchToSide: () => void;
	onResizePointerDown: (event: React.PointerEvent<HTMLDivElement>) => void;
};

export function PanelHost({
	mode,
	sideWidth,
	onSwitchToTop,
	onSwitchToSide,
	onResizePointerDown,
	...controls
}: PanelHostProps) {
	// One control surface, two placements.
	if (mode === "side") {
		return (
			<SidePanel
				{...controls}
				width={sideWidth}
				onSwitchToTop={onSwitchToTop}
				onResizePointerDown={onResizePointerDown}
			/>
		);
	}

	return <TopPanel {...controls} onSwitchToSide={onSwitchToSide} />;
}