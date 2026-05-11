import { ui } from "../../lib/ui";


export function Logo() {
	return (
		<button className={ui.ghostBtn} type="button">
			<img src="/logo.svg" alt="Bonds" className="h-[1.1rem] w-[1.1rem] grayscale contrast-[175%] brightness-50" />
			<span className="font-medium text-lg lg:hidden lg:sr-only">Bonds</span>
		</button>
	)
}