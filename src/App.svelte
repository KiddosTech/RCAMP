<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  type Tab = 'HQ' | 'Settings' | 'Preferences' | 'Plugin' | 'About';
  type Device = { name: string; device_type: string; transport: string; address: string; state: string };
  const tabs: Tab[] = ['HQ', 'Settings', 'Preferences', 'Plugin', 'About'];
  let activeTab: Tab = 'HQ';
  let devices: Device[] = [];
  let loading = false;
  let error = '';
  let adding = false;
  let name = '';
  let address = '';
  let darkMode = true;
  let telemetry = true;

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
</script>

<main class:light={!darkMode}>
  <header class="topbar"><div class="brand"><div class="brand-mark">R</div><div><p class="eyebrow">LOCAL-FIRST HARDWARE CONTROL</p><h1>RCAMP</h1></div></div><div class="top-status"><span class="pulse"></span><span>Core online</span><button onclick={refresh} disabled={loading}>{loading ? 'Refreshing…' : 'Refresh'}</button></div></header>
  <nav class="tabs" aria-label="Application sections">{#each tabs as tab}<button class:active={activeTab === tab} onclick={() => (activeTab = tab)}>{tab === 'HQ' ? '⌂' : tab === 'Plugin' ? '◈' : tab === 'About' ? 'ⓘ' : tab === 'Settings' ? '⚙' : '◌'} <span>{tab}</span></button>{/each}</nav>

  {#if activeTab === 'HQ'}
    <section class="intro"><p class="eyebrow">COMMAND CENTER / 01</p><h2>One controller.<br><em>Many devices.</em></h2><p>Observe, connect, and control your local hardware from one focused workspace.</p></section>
    <form class="add-device" onsubmit={(event) => { event.preventDefault(); void addDevice(); }}><div><label for="device-name">Device name</label><input id="device-name" bind:value={name} required placeholder="ESP32 RC Car" /></div><div><label for="device-address">Network address</label><input id="device-address" bind:value={address} required placeholder="192.168.1.42:80" /></div><button class="primary" disabled={adding}>{adding ? 'Adding…' : 'Add device'}</button></form>
    <section class="devices" aria-label="Devices"><div class="section-title"><div><p class="eyebrow">FLEET</p><h3>Registered devices</h3></div><span>{devices.length} total</span></div>{#if error}<p class="error" role="alert">{error}</p>{:else if devices.length === 0}<div class="empty"><div class="empty-icon">＋</div><h3>No devices connected</h3><p>Add a profile above to begin your local control session.</p></div>{:else}<ul>{#each devices as device}<li><div class="device-icon">⌁</div><div class="device-copy"><strong>{device.name}</strong><span>{device.device_type} · {device.transport}</span></div><div class="status"><span>{device.state}</span><small>{device.address}</small></div></li>{/each}</ul>{/if}</section>
  {:else if activeTab === 'Settings'}
    <section class="panel"><p class="eyebrow">SYSTEM / 02</p><h2>Settings</h2><p class="panel-lead">Configure the RCAMP runtime and local data boundaries.</p><div class="setting-row"><div><strong>Core runtime</strong><span>Shared Rust device manager</span></div><b class="badge good">Online</b></div><div class="setting-row"><div><strong>Profile storage</strong><span>Session storage · durable profiles coming next</span></div><b class="badge">Session</b></div><div class="setting-row"><div><strong>Log level</strong><span>Structured logs from the core</span></div><select aria-label="Log level"><option>Info</option><option>Debug</option><option>Trace</option></select></div></section>
  {:else if activeTab === 'Preferences'}
    <section class="panel"><p class="eyebrow">WORKSPACE / 03</p><h2>Preferences</h2><p class="panel-lead">Tune the interface to your workflow.</p><label class="toggle-row"><span><strong>Dark interface</strong><small>Use the technology-focused dark palette</small></span><input type="checkbox" bind:checked={darkMode} /></label><label class="toggle-row"><span><strong>Live telemetry</strong><small>Show device health when available</small></span><input type="checkbox" bind:checked={telemetry} /></label></section>
  {:else if activeTab === 'Plugin'}
    <section class="panel"><p class="eyebrow">EXTENSIONS / 04</p><h2>Plugins</h2><p class="panel-lead">Extend RCAMP without coupling hardware logic to the interface.</p><div class="plugin-card"><div class="plugin-icon">⌘</div><div><strong>Transport adapters</strong><p>TCP, serial, BLE GATT, and future community transports.</p></div><span class="badge">Core</span></div><div class="plugin-card muted"><div class="plugin-icon">＋</div><div><strong>Community plugins</strong><p>Plugin discovery will arrive with the profile registry.</p></div><span class="badge">Soon</span></div></section>
  {:else}
    <section class="panel about"><div class="about-mark">R</div><p class="eyebrow">RCAMP / 05</p><h2>Hardware, on your terms.</h2><p class="panel-lead">RCAMP is open-source, local-first, and built around one shared Rust core for GUI, Android, and CLI.</p><div class="version">RCAMP <strong>v0.1.0</strong><span>Apache-2.0</span></div></section>
  {/if}
</main>
