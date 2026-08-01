<script lang="ts">
  import { onMount } from 'svelte';
  import { testNimConnection } from '../lib/nim';
  import { DEFAULT_MODEL, type Settings } from '../lib/store';

  let {
    open: isOpen,
    settings,
    busy,
    onClose,
    onSaved,
  }: {
    open: boolean;
    settings: Settings;
    busy: boolean;
    onClose: () => void;
    onSaved: (patch: Partial<Settings>) => void;
  } = $props();

  let apiKey = $state('');
  let model = $state('');
  let showKey = $state(false);
  let testing = $state(false);
  let testResult: 'ok' | 'error' | null = $state(null);
  let testMsg = $state('');

  onMount(() => {
    apiKey = settings.apiKey;
    model = settings.model;
  });

  function handleBackdrop(e: MouseEvent): void {
    if (e.target === e.currentTarget) onClose();
  }

  async function handleTest(): Promise<void> {
    testing = true;
    testResult = null;
    testMsg = '';
    try {
      await testNimConnection(apiKey.trim(), model.trim() || DEFAULT_MODEL);
      testResult = 'ok';
      testMsg = 'Connection OK';
    } catch (err) {
      testResult = 'error';
      testMsg = String(err);
    } finally {
      testing = false;
    }
  }

  function handleSave(): void {
    onSaved({ apiKey: apiKey.trim(), model: model.trim() || DEFAULT_MODEL });
  }
</script>

{#if isOpen}
  <div class="backdrop" role="presentation" onmousedown={handleBackdrop}>
    <div class="panel" role="dialog" aria-modal="true" aria-label="Settings">
      <div class="head">
        <span class="title">Settings</span>
        <button class="close" onclick={onClose} aria-label="Close settings" title="Close">&#10005;</button>
      </div>

      <label class="field">
        <span class="label">NIM API key</span>
        <div class="key-row">
          <input
            type={showKey ? 'text' : 'password'}
            bind:value={apiKey}
            placeholder="nvapi-…"
            spellcheck="false"
          />
          <button class="toggle" onclick={() => (showKey = !showKey)} aria-label="Toggle key visibility">
            {showKey ? 'Hide' : 'Show'}
          </button>
        </div>
        <span class="hint">Get a free key at build.nvidia.com</span>
      </label>

      <label class="field">
        <span class="label">Model</span>
        <div class="key-row">
          <input bind:value={model} class="mono" spellcheck="false" />
          <button class="toggle" onclick={() => (model = DEFAULT_MODEL)} aria-label="Reset model to default" title="Reset to default">
            Reset
          </button>
        </div>
      </label>

      <div class="test-row">
        <button class="btn test" onclick={handleTest} disabled={testing}>
          {#if testing}<span class="spinner"></span>{/if}
          Test connection
        </button>
        {#if testResult === 'ok'}
          <span class="test-msg ok">&#10003; {testMsg}</span>
        {:else if testResult === 'error'}
          <span class="test-msg err">{testMsg}</span>
        {/if}
      </div>

      <div class="foot">
        <button class="btn primary" onclick={handleSave} disabled={busy || testing}>
          Save
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.35);
    z-index: 50;
  }

  .panel {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: min(340px, 90vw);
    background: var(--bg-surface);
    border-left: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    padding: 0 18px 18px;
    animation: slide-in 150ms ease;
  }

  @keyframes slide-in {
    from {
      transform: translateX(24px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: var(--topbar-h);
    flex: none;
  }

  .title {
    font-size: 13px;
    font-weight: 600;
  }

  .close {
    color: var(--text-muted);
    width: 26px;
    height: 26px;
    border-radius: 5px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    transition: color 150ms ease, background-color 150ms ease;
  }

  .close:hover {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 16px;
  }

  .label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .key-row {
    display: flex;
    gap: 6px;
  }

  .key-row input {
    flex: 1;
    min-width: 0;
    font-size: 13px;
  }

  .toggle {
    padding: 0 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-secondary);
    font-size: 12px;
    white-space: nowrap;
    transition: color 150ms ease, border-color 150ms ease;
  }

  .toggle:hover {
    color: var(--accent);
    border-color: var(--accent);
  }

  .hint {
    font-size: 11px;
    color: var(--text-muted);
  }

  .test-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 18px;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 13px;
    border: 1px solid var(--border);
    color: var(--text-primary);
    transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
  }

  .btn:hover:not(:disabled) {
    border-color: var(--accent);
    color: var(--accent);
    background: var(--bg-hover);
  }

  .btn.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }

  .btn.primary:hover:not(:disabled) {
    background: color-mix(in srgb, var(--accent) 88%, #000);
    color: #fff;
  }

  .test-msg {
    font-size: 12px;
    word-break: break-word;
  }

  .test-msg.ok {
    color: var(--success);
  }

  .test-msg.err {
    color: var(--error);
  }

  .foot {
    margin-top: auto;
    display: flex;
    justify-content: flex-end;
  }
</style>
