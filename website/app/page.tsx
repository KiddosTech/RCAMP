export default function Home() {
  return (
    <main>
      <div className="bg-gray-900 min-h-screen py-5">
        <div className="container">
          <div className="row">
            <div className="col-12 text-center">
              <div className="mb-4">
                <i className="iconify text-primary" data-icon="tabler:cpu" style={{ fontSize: '4rem', marginBottom: '1rem' }} />
              </div>
              <h1 className="fw-bold text-white mb-3">One controller</h1>
              <p className="text-muted mb-5">
                Observe, connect, and control local hardware from one focused workspace.
              </p>
              <div className="d-flex gap-3 justify-content-center flex-wrap">
                <a className="btn btn-primary btn-lg" href="/cli/">
                  Install RCAMP/CLI
                </a>
                <a className="btn btn-outline-light text-white btn-lg" href="/docs/">
                  Read the architecture
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="container mt-5">
        <div className="row">
          <div className="col-12 col-md-6">
            <div className="card bg-gray-800 border-gray-600">
              <div className="card-body">
                <span className="text-primary small">REGISTERED DEVICES</span>
                <h3 className="fw-bold">0</h3>
                <small>Ready for your first profile</small>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6">
            <div className="card bg-gray-800 border-gray-600">
              <div className="card-body">
                <span className="text-primary small">TRANSPORTS</span>
                <h3 className="fw-bold">05</h3>
                <small className="text-muted">TCP · UDP · BLE · USB · Serial</small>
              </div>
            </div>
          </div>
        </div>
        <div className="row mt-4">
          <div className="col-12">
            <div className="card bg-gray-800 border-gray-600">
              <div className="card-body">
                <span className="text-primary small">RUNTIME</span>
                <h3 className="fw-bold">Online</h3>
                <small>Shared Rust core v0.1.0</small>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}