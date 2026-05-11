import { ui } from "../../lib/ui";


export function Logo() {
	return (
		<button className={ui.ghostBtn} type="button">
			<img src="/logo.svg" alt="Bonds" className="h-[1.05rem] w-[1.05rem] grayscale contrast-75 brightness-50" />
			<span className="font-medium text-lg lg:hidden lg:sr-only">Bonds</span>
		</button>
	)
}