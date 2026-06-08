/**
 * Icons.tsx
 *
 * This file defines a set of reusable icon components for the application. Each icon is represented as an <img> element with a source pointing to the corresponding SVG file in the public directory. The components accept an optional `className` prop to allow for custom styling and integration with Tailwind CSS classes.
 *
 * @see http://pixeliconlibrary.com for the source of the SVG icons used in this application.
 */

import { cx } from "../../lib/ui";

interface IconProps {
	className?: string;
}

const IconWrapper = ({ className, children, ...props }: IconProps & { children: React.ReactNode }) => {
	return <div className={cx(className, "w-5 h-5 *:fill-inherit shadow-xl")} {...props}>{children}</div>;
}

export const SidePanelOpenIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="side-nav-collapse-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M22,5V3H20V2H4V3H2V5H1V19H2v2H4v1H20V21h2V19h1V5ZM20,18H19v1H10V5h9V6h1Z" /><polygon points="18 7 18 9 17 9 17 10 16 10 16 11 15 11 15 13 16 13 16 14 17 14 17 15 18 15 18 17 16 17 16 16 15 16 15 15 14 15 14 14 13 14 13 13 12 13 12 11 13 11 13 10 14 10 14 9 15 9 15 8 16 8 16 7 18 7" /></svg></IconWrapper>;

export const SidePanelCloseIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="side-nav-expand-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M22,5V3H20V2H4V3H2V5H1V19H2v2H4v1H20V21h2V19h1V5ZM20,18H19v1H10V5h9V6h1Z" /><polygon points="18 11 18 13 17 13 17 14 16 14 16 15 15 15 15 16 14 16 14 17 12 17 12 15 13 15 13 14 14 14 14 13 15 13 15 11 14 11 14 10 13 10 13 9 12 9 12 7 14 7 14 8 15 8 15 9 16 9 16 10 17 10 17 11 18 11" /></svg></IconWrapper>;

export const TopPanelOpenIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}>
	<SidePanelOpenIcon className="rotate-90" />
</IconWrapper>;

export const TopPanelCloseIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}>
	<SidePanelCloseIcon className="rotate-90" />
</IconWrapper>;

export const HomeIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="home-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><polygon points="23 11 23 12 20 12 20 22 19 22 19 23 15 23 15 16 9 16 9 23 5 23 5 22 4 22 4 12 1 12 1 11 2 11 2 10 3 10 3 9 4 9 4 8 5 8 5 7 6 7 6 6 7 6 7 5 8 5 8 4 9 4 9 3 10 3 10 2 11 2 11 1 13 1 13 2 14 2 14 3 15 3 15 4 16 4 16 5 17 5 17 6 18 6 18 7 19 7 19 8 20 8 20 9 21 9 21 10 22 10 22 11 23 11" /></svg></IconWrapper>;

export const RefreshIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="refresh-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><polygon points="23 14 23 15 22 15 22 17 21 17 21 19 20 19 20 20 19 20 19 21 17 21 17 22 15 22 15 23 9 23 9 22 7 22 7 21 5 21 5 20 3 20 3 21 2 21 2 22 1 22 1 14 9 14 9 15 8 15 8 16 7 16 7 18 8 18 8 19 10 19 10 20 14 20 14 19 16 19 16 18 17 18 17 17 18 17 18 15 19 15 19 14 23 14" /><polygon points="23 2 23 10 15 10 15 9 16 9 16 8 17 8 17 6 16 6 16 5 14 5 14 4 10 4 10 5 8 5 8 6 7 6 7 7 6 7 6 9 5 9 5 10 1 10 1 9 2 9 2 7 3 7 3 5 4 5 4 4 5 4 5 3 7 3 7 2 9 2 9 1 15 1 15 2 17 2 17 3 19 3 19 4 21 4 21 3 22 3 22 2 23 2" /></svg></IconWrapper>;

export const AddIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="plus-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><polygon points="23 11 23 13 22 13 22 14 14 14 14 22 13 22 13 23 11 23 11 22 10 22 10 14 2 14 2 13 1 13 1 11 2 11 2 10 10 10 10 2 11 2 11 1 13 1 13 2 14 2 14 10 22 10 22 11 23 11" /></svg></IconWrapper>;

export const DeleteIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="trash-alt-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><polygon points="22 3 22 5 2 5 2 3 8 3 8 2 9 2 9 1 15 1 15 2 16 2 16 3 22 3" /><path d="m4,7v15h1v1h14v-2h1V7H4Zm12,12h-2v-10h2v10Zm-6,0h-2v-10h2v10Z" /></svg></IconWrapper>;

export const EditIcon = ({ className, ...props }: IconProps) =>
	<IconWrapper className={className} {...props}><svg id="edit-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><polygon points="22 4 22 7 21 7 21 8 20 8 20 7 19 7 19 6 18 6 18 5 17 5 17 4 18 4 18 3 21 3 21 4 22 4" /><polygon points="17 14 18 14 18 21 17 21 17 22 2 22 2 21 1 21 1 6 2 6 2 5 14 5 14 6 13 6 13 7 3 7 3 20 16 20 16 15 17 15 17 14" /><polygon points="18 8 19 8 19 10 18 10 18 11 17 11 17 12 16 12 16 13 15 13 15 14 14 14 14 15 13 15 13 16 12 16 12 17 11 17 11 18 7 18 7 14 8 14 8 13 9 13 9 12 10 12 10 11 11 11 11 10 12 10 12 9 13 9 13 8 14 8 14 7 15 7 15 6 17 6 17 7 18 7 18 8" /></svg>
	</IconWrapper>;

export const SnapshotIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}	><svg id="camera" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M22,7V6H21V5H17V3H16V2H8V3H7V5H3V6H2V7H1V20H2v1H3v1H21V21h1V20h1V7ZM21,19H20v1H4V19H3V8H4V7H8V6H9V4h6V6h1V7h4V8h1Z" /><polygon points="16 11 16 15 15 15 15 16 14 16 14 17 10 17 10 16 9 16 9 15 8 15 8 11 9 11 9 10 10 10 10 9 14 9 14 10 15 10 15 11 16 11" /><rect x="5" y="8" width="2" height="2" /></svg></IconWrapper>;

export const CancelIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="times-square" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M22,2V1H2V2H1V22H2v1H22V22h1V2ZM21,21H3V3H21Z" /><polygon points="14 13 15 13 15 14 16 14 16 15 17 15 17 16 16 16 16 17 15 17 15 16 14 16 14 15 13 15 13 14 11 14 11 15 10 15 10 16 9 16 9 17 8 17 8 16 7 16 7 15 8 15 8 14 9 14 9 13 10 13 10 11 9 11 9 10 8 10 8 9 7 9 7 8 8 8 8 7 9 7 9 8 10 8 10 9 11 9 11 10 13 10 13 9 14 9 14 8 15 8 15 7 16 7 16 8 17 8 17 9 16 9 16 10 15 10 15 11 14 11 14 13" /></svg></IconWrapper>;

export const RestoreIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="merge-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M21,11V10H20V9H17v1H16v1H11V10H9V9H8V8H7V7H8V6H9V3H8V2H7V1H4V2H3V3H2V6H3V7H4V17H3v1H2v3H3v1H4v1H7V22H8V21H9V18H8V17H7V11H8v1H9v1h2v1h5v1h1v1h3V15h1V14h1V11ZM5,4V3H6V4H7V5H6V6H5V5H4V4ZM6,20v1H5V20H4V19H5V18H6v1H7v1Zm13-7v1H18V13H17V12h1V11h1v1h1v1Z" /></svg></IconWrapper>;

export const HistoryIcon = ({ className, ...props }: IconProps) => <IconWrapper className={className} {...props}><svg id="clock-solid" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="m22,9v-2h-1v-2h-1v-1h-1v-1h-2v-1h-2v-1h-6v1h-2v1h-2v1h-1v1h-1v2h-1v2h-1v6h1v2h1v2h1v1h1v1h2v1h2v1h6v-1h2v-1h2v-1h1v-1h1v-2h1v-2h1v-6h-1Zm-9,7v-1h-1v-1h-1V5h2v8h1v1h1v1h1v1h-1v1h-1v-1h-1Z" /></svg></IconWrapper>;