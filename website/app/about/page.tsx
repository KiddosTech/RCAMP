export default function About() {
  return (
    <main>
      <div className="py-5">
        <div className="container">
          <div className="row">
            <div className="col-12 text-center">
              <span className="text-primary small fw-normal">ABOUT RCAMP</span>
              <h2 className="fw-bold text-white mb-3">Hardware, <span className="text-muted">on your terms.</span></h2>
              <p className="text-muted lead mb-4">
                RCAMP is an open-source, local-first control plane built around one shared Rust core for GUI, Android, CLI, and embedded targets.
              </p>
            </div>
          </div>
        </div>
      </div>
      <div className="container">
        <div className="row g-4">
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:device-desktop" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">One shared core</h3>
                <p className="card-text text-muted">
                  Device management and transport logic live in Rust, not duplicated inside each interface.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:shield-lock" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Private by default</h3>
                <p className="card-text text-muted">
                  No mandatory cloud account. Hardware traffic stays on your local network.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:topology-star3" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Capability-driven</h3>
                <p className="card-text text-muted">
                  Profiles describe what a device can do without assuming every board speaks the same protocol.
                </p>
              </div>
            </div>
          </div>
        </div>
        <section className="mt-5 text-center">
          <div className="container">
            <span className="text-primary small fw-normal">PROJECT VALUES</span>
            <h3 className="card-title fw-bold text-white">Small surface. Serious control.</h3>
            <p className="text-muted">
              RCAMP connects Wi-Fi, BLE, USB, serial, and future community transports behind clear APIs. The GUI, RCAMP/CLI, and RCAMP/RTOS remain separate experiences with one source of truth.
            </p>
          </div>
        </section>
      </div>
    </main>
  );
}