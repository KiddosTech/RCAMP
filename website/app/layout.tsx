import type { Metadata } from 'next';
import './globals.css';
import { IconActivity, IconCpu, IconDeviceDesktop, IconFileText, IconSettings, IconTerminal2 } from '@tabler/icons-react';

export const metadata: Metadata = {
  title: 'RCAMP — Hardware control without the cloud',
  description: 'Open-source, local-first hardware control for desktop, mobile, and embedded targets.',
  icons: { icon: '/assets/rcamp-mark.svg' }
};

const nav = [
  ['Overview', '/', IconActivity],
  ['Devices', '/about/', IconDeviceDesktop],
  ['RCAMP/CLI', '/cli/', IconTerminal2],
  ['Documentation', '/docs/', IconFileText],
  ['Settings', '/about/#settings', IconSettings]
] as const;

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body><div className="app-shell"><aside className="sidebar"><a className="brand" href="/"><img src="/assets/rcamp-mark.svg" alt="RCAMP" /><span>RCAMP</span></a><p className="side-label">CONTROL PLANE</p><nav>{nav.map(([label, href, Icon]) => <a href={href} key={label}><Icon size={18} stroke={1.8} /><span>{label}</span></a>)}</nav><div className="sidebar-bottom"><span className="online-dot" /> Core online <small>v0.1.0</small></div></aside><div className="workspace"><header className="topbar"><div><span className="breadcrumb">RCAMP / LOCAL CONTROL</span><h1>Hardware workspace</h1></div><a className="github-button" href="https://github.com/KiddosTech/RCAMP"><img src="https://cdn.simpleicons.org/github/ffffff" alt="" /> GitHub</a></header>{children}</div></div></body></html>;
}
