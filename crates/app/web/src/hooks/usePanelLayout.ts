import { useCallback, useEffect, useState } from "react";
import {
	COMPACT_BREAKPOINT,
	DEFAULT_LAYOUT,
	LAYOUT_STORAGE_KEY,
	clampSideWidth,
} from "../lib/layout";
import type { PanelMode, ViewTab } from "../lib/types";
import { usePersistedState } from "./usePersistedState";

function isCompactViewport(): boolean {
	if (typeof window === "undefined") return false;
	return window.innerWidth < COMPACT_BREAKPOINT;
}

export function usePanelLayout() {
	const [layout, setLayout] = usePersistedState(LAYOUT_STORAGE_KEY, DEFAULT_LAYOUT);
	const [isCompact, setIsCompact] = useState<boolean>(isCompactViewport);

	useEffect(() => {
		const onResize = () => setIsCompact(isCompactViewport());
		window.addEventListener("resize", onResize);
		return () => window.removeEventListener("resize", onResize);
	}, []);

	const setMode = useCallback((mode: PanelMode) => {
		setLayout((prev) => ({ ...prev, mode, isOpen: true }));
	}, [setLayout]);

	const togglePanel = useCallback(() => {
		setLayout((prev) => ({ ...prev, isOpen: !prev.isOpen }));
	}, [setLayout]);

	const setSearch = useCallback((search: string) => {
		setLayout((prev) => ({ ...prev, search }));
	}, [setLayout]);

	const setActiveTab = useCallback((activeTab: ViewTab) => {
		setLayout((prev) => ({ ...prev, activeTab }));
	}, [setLayout]);

	const setSideWidth = useCallback((sideWidth: number) => {
		setLayout((prev) => ({ ...prev, sideWidth: clampSideWidth(sideWidth) }));
	}, [setLayout]);

	return {
		layout,
		isCompact,
		setMode,
		togglePanel,
		setSearch,
		setActiveTab,
		setSideWidth,
	};
}