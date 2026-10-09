<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import IconAdjustments from '@tabler/icons-svelte/icons/adjustments';
  import IconCpu from '@tabler/icons-svelte/icons/cpu';
  import IconDeviceDesktopAnalytics from '@tabler/icons-svelte/icons/device-desktop-analytics';
  import IconHome2 from '@tabler/icons-svelte/icons/home-2';
  import IconInfoCircle from '@tabler/icons-svelte/icons/info-circle';
  import IconLogs from '@tabler/icons-svelte/icons/logs';
  import IconPlugConnected from '@tabler/icons-svelte/icons/plug-connected';
  import IconRefresh from '@tabler/icons-svelte/icons/refresh';
  import IconSettings from '@tabler/icons-svelte/icons/settings';
  import IconTerminal2 from '@tabler/icons-svelte/icons/terminal-2';
  import IconTool from '@tabler/icons-svelte/icons/tool';
  import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
  import IconPlayerStop from '@tabler/icons-svelte/icons/player-stop';

  type Tab = 'HQ' | 'Simulator' | 'Shell' | 'Logs' | 'Tools' | 'Settings' | 'Preferences' | 'Plugin' | 'About';
  type Device = { name: string; device_type: string; transport: string; address: string; state: string };
  const tabs: Tab[] = ['HQ', 'Simulator', 'Shell', 'Logs', 'Tools', 'Settings', 'Preferences', 'Plugin', 'About'];
  let activeTab: Tab = 'HQ';
  let devices: Device[] = [];
  let loading = false;
  let error = '';
  let adding = false;
  let name = '';
  let address = '';
  let darkMode = true;
  let telemetry = true;
  let flashTarget = 'esp32';
  let flashPort = '';
  let firmwarePath = '';
  let flashing = false;
  let flashMessage = '';
  type LogEntry = { timestamp: number; level: string; message: string };
  let logs: LogEntry[] = [];
  let shellAddress = '';
  let shellInput = 'help';
  let shellOutput = '';
  let shellBusy = false;
  let simulatorRunning = false;
  let simulatorCommand = 'rcampfetch';
  let simulatorOutput = 'ESP32 simulation is ready. Start the virtual target to begin.';
  let simulatorLogs: string[] = [];
  const simulatorTasks = ['rgb_on', 'rgb_off', 'rgb_set 24 96 255', 'led_on', 'led_off', 'buzzer_beep', 'relay_on', 'relay_off', 'test_write', 'read_input', 'sensor_read', 'status_snapshot', 'telemetry_on', 'identify', 'uptime', 'storage_info', 'reboot'];

  async function refresh() {
    loading = true; error = '';
    try { devices = await invoke<Device[]>('list_devices'); }
    catch (reason) { error = `Unable to load devices: ${String(reason)}`; }
    finally { loading = false; }
  }

  async function addDevice() {
    adding = true; error = '';
    try {
      const device = await invoke<Device>('add_device', { profile: { name, type: 'custom-device', connection: 'tcp', address, capabilities: [], commands: [], controls: {}, metadata: {} } });
      devices = [...devices, device]; name = ''; address = '';
    } catch (reason) { error = `Unable to add device: ${String(reason)}`; }
    finally { adding = false; }
  }

  async function flashFirmware() {
    flashing = true; flashMessage = '';
    try {
      flashMessage = await invoke<string>('flash_target', { target: flashTarget, port: flashPort, firmware: firmwarePath });
    } catch (reason) { flashMessage = String(reason); }
    finally { flashing = false; }
  }

  async function runShell() {
    shellBusy = true; shellOutput = '';
    try { shellOutput = await invoke<string>('shell_command', { address: shellAddress, command: shellInput }); }
    catch (reason) { shellOutput = `ERROR: ${String(reason)}`; }
    finally { shellBusy = false; void loadLogs(); }
  }

  async function loadLogs() {
    try { logs = await invoke<LogEntry[]>('list_logs'); } catch { /* unavailable before Tauri starts */ }
  }

  function toggleSimulator() {
    simulatorRunning = !simulatorRunning;
    simulatorOutput = simulatorRunning
      ? 'RCAMP/RTOS virtual target online\ntransport=simulator\ntarget=ESP32 DevKit\nready for commands'
      : 'ESP32 simulation stopped.';
    simulatorLogs = [...simulatorLogs, simulatorRunning ? 'virtual ESP32 connected' : 'virtual ESP32 disconnected'];
  }

  function runSimulation() {
    if (!simulatorRunning) { simulatorOutput = 'Start the ESP32 simulation first.'; return; }
    const command = simulatorCommand.trim();
    const lower = command.toLowerCase();
    if (lower === 'rcampfetch') {
      simulatorOutput = 'RCAMPFETCH\nrcamp=0.1.0\nrole=Administrator\nhost=RCAMP Simulator\ntarget=ESP32 DevKit\ntransport=virtual-tcp\nheap=281 KB\nstatus=ready';
    } else if (lower === 'help') {
      simulatorOutput = `RCAMP/RTOS virtual shell\ncommands: help status info capabilities rcampfetch task <name>\n\ntasks: ${simulatorTasks.join(' · ')}`;
    } else if (lower.startsWith('task ')) {
      const task = command.slice(5).trim();
      simulatorOutput = simulatorTasks.some((item) => item.split(' ')[0] === task)
        ? `[simulator] task ${task} completed\nresult=ok\ntarget=ESP32 DevKit`
        : `[simulator] unknown task: ${task}`;
    } else if (['status', 'info', 'capabilities'].includes(lower)) {
      simulatorOutput = `[simulator] ${lower}\nstate=ready\nfirmware=RCAMP/RTOS 0.1.0\ncapabilities=tasks,telemetry,shell`;
    } else {
      simulatorOutput = `[simulator] command not available: ${command}`;
    }
    simulatorLogs = [...simulatorLogs, `${new Date().toLocaleTimeString()}  ${command}`];
  }
</script>

<main class:light={!darkMode}>
  <header class="topbar"><div class="brand"><img class="brand-mark" src="/rcamp-mark.svg" alt="RCAMP" /><div><p class="eyebrow">LOCAL-FIRST HARDWARE CONTROL</p><h1>RCAMP</h1></div></div><div class="top-status"><span class="pulse"></span><span>Core online</span><button class="icon-button" onclick={refresh} disabled={loading} aria-label="Refresh devices"><IconRefresh size={17} stroke={1.8} class={loading ? 'spin' : ''} /></button></div></header>
  <nav class="tabs" aria-label="Application sections">{#each tabs as tab}<button class:active={activeTab === tab} onclick={() => { activeTab = tab; if (tab === 'Logs') void loadLogs(); }}>{#if tab === 'HQ'}<IconHome2 size={16} stroke={1.8} />{:else if tab === 'Simulator'}<IconCpu size={16} stroke={1.8} />{:else if tab === 'Shell'}<IconTerminal2 size={16} stroke={1.8} />{:else if tab === 'Logs'}<IconLogs size={16} stroke={1.8} />{:else if tab === 'Tools'}<IconTool size={16} stroke={1.8} />{:else if tab === 'Plugin'}<IconPlugConnected size={16} stroke={1.8} />{:else if tab === 'About'}<IconInfoCircle size={16} stroke={1.8} />{:else if tab === 'Settings'}<IconSettings size={16} stroke={1.8} />{:else}<IconAdjustments size={16} stroke={1.8} />{/if}<span>{tab}</span></button>{/each}</nav>

  {#if activeTab === 'HQ'}
    <section class="intro"><p class="eyebrow">COMMAND CENTER / 01</p><h2>One controller.<br><em>Many devices.</em></h2><p>Observe, connect, and control your local hardware from one focused workspace.</p></section>
    <form class="add-device" onsubmit={(event) => { event.preventDefault(); void addDevice(); }}><div><label for="device-name">Device name</label><input id="device-name" bind:value={name} required placeholder="ESP32 RC Car" /></div><div><label for="device-address">Network address</label><input id="device-address" bind:value={address} required placeholder="192.168.1.42:80" /></div><button class="primary" disabled={adding}>{adding ? 'Adding…' : 'Add device'}</button></form>
    <section class="devices" aria-label="Devices"><div class="section-title"><div><p class="eyebrow">FLEET</p><h3>Registered devices</h3></div><span>{devices.length} total</span></div>{#if error}<p class="error" role="alert">{error}</p>{:else if devices.length === 0}<div class="empty"><div class="empty-icon">＋</div><h3>No devices connected</h3><p>Add a profile above to begin your local control session.</p></div>{:else}<ul>{#each devices as device}<li><div class="device-icon">⌁</div><div class="device-copy"><strong>{device.name}</strong><span>{device.device_type} · {device.transport}</span></div><div class="status"><span>{device.state}</span><small>{device.address}</small></div></li>{/each}</ul>{/if}</section>
  {:else if activeTab === 'Simulator'}
    <section class="panel simulator-panel"><div class="simulator-heading"><div><p class="eyebrow">ESP32 SIMULATION / 02</p><h2>Trial RCAMP/RTOS safely.</h2><p class="panel-lead">Run a virtual ESP32 target without hardware, serial drivers, Wi-Fi, or flashing. Test the shell, task catalog, telemetry flow, and UI before connecting a real board.</p></div><div class="simulator-chip"><IconDeviceDesktopAnalytics size={18} stroke={1.8} /><span>{simulatorRunning ? 'Virtual target online' : 'Stopped'}</span></div></div><div class="simulator-card"><div class="simulator-toolbar"><div><strong>ESP32 DevKit</strong><span>RCAMP/RTOS 0.1.0 · virtual-tcp</span></div><button class={simulatorRunning ? 'danger' : 'primary'} onclick={toggleSimulator}>{#if simulatorRunning}<IconPlayerStop size={16} stroke={1.8} />Stop simulation{:else}<IconPlayerPlay size={16} stroke={1.8} />Start simulation{/if}</button></div><div class="simulator-command"><label for="sim-command">Virtual shell command<input id="sim-command" bind:value={simulatorCommand} onkeydown={(event) => { if (event.key === 'Enter') runSimulation(); }} placeholder="rcampfetch or task rgb_on" /></label><button class="primary" onclick={runSimulation} disabled={!simulatorRunning}>Run</button></div><pre class="terminal-output">{simulatorOutput}</pre><div class="simulator-footer"><div><span class="eyebrow">TASK CATALOG</span><div class="task-chips">{#each simulatorTasks as task}<code>{task}</code>{/each}</div></div><div class="simulator-log"><span class="eyebrow">SIMULATION LOG</span>{#if simulatorLogs.length === 0}<small>No virtual events yet.</small>{:else}{#each simulatorLogs.slice(-4) as item}<small>{item}</small>{/each}{/if}</div></div></div></section>
  {:else if activeTab === 'Shell'}
    <section class="panel shell-panel"><p class="eyebrow">RCAMP/RTOS SHELL / 02</p><h2>Talk to the target.</h2><p class="panel-lead">A realtime, line-flushed simulated shell for safe RCAMP/RTOS commands. This never executes commands on your PC.</p><div class="shell-form"><label for="shell-address">Target address<input id="shell-address" bind:value={shellAddress} required placeholder="192.168.1.42:8080" /></label><label for="shell-command">Command<input id="shell-command" bind:value={shellInput} required placeholder="rcampfetch" /></label><button class="primary" onclick={() => void runShell()} disabled={shellBusy}>{shellBusy ? 'Waiting…' : 'Send command'}</button></div><pre class="terminal-output">{shellOutput || 'RCAMP/RTOS target output will appear here without application buffering.'}</pre><p class="hint">Tasks: <code>rgb_on</code> · <code>rgb_off</code> · <code>rgb_set</code> · <code>led_on</code> · <code>led_off</code> · <code>buzzer_beep</code> · <code>relay_on</code> · <code>relay_off</code> · <code>test_write</code> · <code>read_input</code> · <code>sensor_read</code> · <code>status_snapshot</code> · <code>telemetry_on/off</code> · <code>identify</code> · <code>uptime</code> · <code>storage_info</code> · <code>reboot</code></p></section>
  {:else if activeTab === 'Logs'}
    <section class="panel"><p class="eyebrow">SYSTEM LOG / 03</p><h2>Event stream.</h2><p class="panel-lead">Structured local events from device registration, shell traffic, flashing, and connection errors.</p><div class="log-list">{#if logs.length === 0}<div class="empty"><h3>No events yet</h3><p>Actions from the app and target will appear here.</p></div>{:else}{#each logs as entry}<div class="log-entry"><span class="log-level {entry.level}">{entry.level}</span><span>{entry.message}</span><time>{new Date(entry.timestamp * 1000).toLocaleTimeString()}</time></div>{/each}{/if}</div></section>
  {:else if activeTab === 'Tools'}
    <section class="panel"><p class="eyebrow">TARGET TOOLS / 02</p><h2>Flash RCAMP/RTOS.</h2><p class="panel-lead">Install the RCAMP/RTOS firmware on an ESP32 or Arduino target. RCAMP calls the native vendor flasher locally—no firmware or device data is uploaded.</p><div class="flash-form"><label for="flash-target">Target<select id="flash-target" bind:value={flashTarget}><option value="esp32">ESP32 · esptool</option><option value="arduino">Arduino · avrdude</option></select></label><label for="flash-port">Serial port<input id="flash-port" bind:value={flashPort} required placeholder="COM3 or /dev/ttyUSB0" /></label><label for="firmware-path">Firmware file<input id="firmware-path" bind:value={firmwarePath} required placeholder="C:\\firmware\\rcamp-rtos.bin" /></label><button class="primary" onclick={() => void flashFirmware()} disabled={flashing}>{flashing ? 'Flashing…' : 'Flash target'}</button></div>{#if flashMessage}<pre class="flash-output">{flashMessage}</pre>{/if}<div class="tool-note"><strong>Required tools</strong><span>ESP32: <code>esptool</code> · Arduino: <code>avrdude</code> available on PATH</span></div></section>
  {:else if activeTab === 'Settings'}
    <section class="panel"><p class="eyebrow">SYSTEM / 02</p><h2>Settings</h2><p class="panel-lead">Configure the RCAMP runtime and local data boundaries.</p><div class="setting-row"><div><strong>Core runtime</strong><span>Shared Rust device manager</span></div><b class="badge good">Online</b></div><div class="setting-row"><div><strong>Profile storage</strong><span>Session storage · durable profiles coming next</span></div><b class="badge">Session</b></div><div class="setting-row"><div><strong>Log level</strong><span>Structured logs from the core</span></div><select aria-label="Log level"><option>Info</option><option>Debug</option><option>Trace</option></select></div></section>
  {:else if activeTab === 'Preferences'}
    <section class="panel"><p class="eyebrow">WORKSPACE / 03</p><h2>Preferences</h2><p class="panel-lead">Tune the interface to your workflow.</p><label class="toggle-row"><span><strong>Dark interface</strong><small>Use the technology-focused dark palette</small></span><input type="checkbox" bind:checked={darkMode} /></label><label class="toggle-row"><span><strong>Live telemetry</strong><small>Show device health when available</small></span><input type="checkbox" bind:checked={telemetry} /></label></section>
  {:else if activeTab === 'Plugin'}
    <section class="panel"><p class="eyebrow">EXTENSIONS / 04</p><h2>Plugins</h2><p class="panel-lead">Extend RCAMP without coupling hardware logic to the interface.</p><div class="plugin-card"><div class="plugin-icon">⌘</div><div><strong>Transport adapters</strong><p>TCP, serial, BLE GATT, and future community transports.</p></div><span class="badge">Core</span></div><div class="plugin-card muted"><div class="plugin-icon">＋</div><div><strong>Community plugins</strong><p>Plugin discovery will arrive with the profile registry.</p></div><span class="badge">Soon</span></div></section>
  {:else}
    <section class="panel about"><img class="about-mark" src="/rcamp-mark.svg" alt="RCAMP logo" /><p class="eyebrow">RCAMP / 05</p><h2>Hardware, on your terms.</h2><p class="panel-lead">RCAMP is open-source, local-first, and built around one shared Rust core for GUI, Android, and CLI.</p><div class="version">RCAMP <strong>v0.1.0</strong><span>Apache-2.0</span></div></section>
  {/if}
</main>
