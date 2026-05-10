import type { PropsWithChildren } from "react";

export function MainViewport({ children }: PropsWithChildren) {
	return <section className="p-4 md:p-5">{children}</section>;
}