import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'RCAMP — Hardware control without the cloud',
  description: 'Open-source, local-first hardware control for desktop, mobile, and embedded targets.',
  icons: { icon: '/assets/rcamp-mark.svg' }
};

const nav = [
  ['Overview', '/', 'activity'],
  ['Devices', '/about/', 'device-desktop'],
  ['RCAMP/CLI', '/cli/', 'terminal2'],
  ['Documentation', '/docs/', 'file-text'],
  ['Settings', '/about/#settings', 'settings']
] as const;

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body className="bg-gray-900 min-h-screen text-white">
        <div className="container-fluid">
          <div className="row">
            <div className="col-lg-2 col-md-3 col-sm-4 sidebar bg-gray-800 sticky-top">
              <a className="brand" href="/">
                <img src="/assets/rcamp-mark.svg" width="32" alt="RCAMP" />
                <span>RCAMP</span>
              </a>
              <p className="side-label text-muted fw-normal small">CONTROL PLANE</p>
              <nav className="nav mt-3">
                {nav.map(([label, href, iconName]) => (
                  <a href={href} key={label} className="nav-link text-muted small">
                    <i className="iconify text-primary" data-icon={`tabler:${iconName}`} style={{ width: 18, height: 18 }} />
                    <span>{label}</span>
                  </a>
                ))}
              </nav>
              <div className="sidebar-bottom mt-auto">
                <span className="online-dot" /> Core online <small>v0.1.0</small>
              </div>
            </div>
            <div className="col-lg-10 col-md-9 col-sm-8 workspace p-4 p-lg-5 min-vh-100">
              <header className="topbar mb-3 mb-lg-4 border-bottom border-gray-600">
                <div className="d-flex justify-content-between">
                  <span className="breadcrumb text-muted">RCAMP / LOCAL CONTROL</span>
                  <h1 className="mb-0">Hardware workspace</h1>
                </div>
                <a
                  className="btn btn-outline-light text-light"
                  href="https://github.com/KiddosTech/RCAMP"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <img
                    src="https://cdn.simpleicons.org/github/ffffff"
                    width="16"
                    alt="GitHub"
                    style={{ marginRight: 8 }}
                  />
                  GitHub
                </a>
              </header>
            {children}
            </div>
          </div>
        </div>
      </body>
    </html>
  );
}