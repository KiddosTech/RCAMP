<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  type Device = { name: string; device_type: string; transport: string; address: string; state: string };
  let devices: Device[] = [];
  let loading = false;
  let error = '';
  let adding = false;
  let name = '';
  let address = '';

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

<main>
  <header><div><p class="eyebrow">LOCAL-FIRST HARDWARE CONTROL</p><h1>RCAMP</h1></div><button onclick={refresh} disabled={loading}>{loading ? 'Refreshing…' : 'Refresh devices'}</button></header>
  <section class="intro"><h2>One controller. Many devices.</h2><p>Manage hardware through Wi-Fi, Bluetooth and USB without a cloud account.</p></section>
  <form class="add-device" onsubmit={(event) => { event.preventDefault(); void addDevice(); }}><input bind:value={name} required placeholder="Device name" aria-label="Device name" /><input bind:value={address} required placeholder="Address, e.g. 192.168.1.42:80" aria-label="Device address" /><button disabled={adding}>{adding ? 'Adding…' : 'Add TCP device'}</button></form>
  <section class="devices" aria-label="Devices"><div class="section-title"><h2>Devices</h2><span>{devices.length} registered</span></div>
    {#if error}<p class="error" role="alert">{error}</p>
    {:else if devices.length === 0}<div class="empty"><h3>No devices yet</h3><p>Create a local device profile to get started.</p></div>
    {:else}<ul>{#each devices as device}<li><div><strong>{device.name}</strong><span>{device.device_type} · {device.transport}</span></div><div class="status"><span>{device.state}</span><small>{device.address}</small></div></li>{/each}</ul>{/if}
  </section>
</main>
