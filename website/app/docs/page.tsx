export default function Docs() {
  return (
    <main>
      <div className="py-5">
        <div className="container">
          <div className="row">
            <div className="col-12 text-center">
              <span className="text-primary small fw-normal">DOCUMENTATION</span>
              <h2 className="fw-bold text-white mb-3">Understand the <span className="text-muted">control plane.</span></h2>
              <p className="text-muted lead">
                Start with profiles, connect a transport, then let the shared core expose safe capabilities to every interface.
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
                <i className="iconify text-primary" data-icon="tabler:binary-tree" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Architecture</h3>
                <p className="card-text text-muted">
                  GUI and CLI call rcamp-core; transports stay behind the device manager.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:file-code" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Device profiles</h3>
                <p className="card-text text-muted">
                  Describe name, type, address, capabilities, commands, controls, and metadata.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:broadcast" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Transports</h3>
                <p className="card-text text-muted">
                  Use TCP, UDP, HTTP, WebSocket, MQTT, BLE GATT, USB, and serial.
                </p>
              </div>
            </div>
          </div>
        </div>
        <div className="row g-4">
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:cpu" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">RCAMP/RTOS</h3>
                <p className="card-text text-muted">
                  Reference ESP32 and Arduino targets with a line-oriented shell.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:player-play" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">ESP32 simulator</h3>
                <p className="card-text text-muted">
                  Trial shell commands and task capabilities before plugging in hardware.
                </p>
              </div>
            </div>
          </div>
          <div className="col-12 col-md-6 col-lg-4">
            <div className="card bg-gray-800 border-gray-600 h-100">
              <div className="card-body">
                <i className="iconify text-primary" data-icon="tabler:api" style={{ fontSize: '2rem', marginBottom: '1rem', display: 'block', textAlign: 'center' }} />
                <h3 className="card-title fw-bold">Core API</h3>
                <p className="card-text text-muted">
                  Keep hardware input validated, local-first, and capability-driven.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}