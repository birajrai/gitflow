<script lang="ts">
  let { disabled, onSend }: { disabled: boolean; onSend: (text: string) => void } = $props();

  let value = $state('');
  let textarea: HTMLTextAreaElement;

  function autogrow(): void {
    if (!textarea) return;
    textarea.style.height = 'auto';
    textarea.style.height = Math.min(textarea.scrollHeight, 63) + 'px';
  }

  function handleKeydown(e: KeyboardEvent): void {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  }

  function send(): void {
    const v = value.trim();
    if (!v || disabled) return;
    onSend(v);
    value = '';
    autogrow();
  }
</script>

<div class="input-row">
  <textarea
    class="input mono"
    bind:this={textarea}
    bind:value
    rows="1"
    placeholder='Ask anything or give a command — try "commit and push"'
    disabled={disabled}
    onkeydown={handleKeydown}
    oninput={autogrow}
  ></textarea>
  <button class="send" onclick={send} disabled={disabled || !value.trim()} aria-label="Send" title="Send">
    <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round">
      <path d="M2 8 14 2.5 10.5 14 8 9.5 2 8Z" />
      <path d="M8 9.5 14 2.5" />
    </svg>
  </button>
</div>

<style>
  .input-row {
    display: flex;
    align-items: flex-end;
    gap: 8px;
    padding: 10px 18px 12px;
    border-top: 1px solid var(--border-subtle);
    background: var(--bg-base);
    flex: none;
  }

  .input {
    flex: 1;
    resize: none;
    overflow-y: auto;
    max-height: 63px;
    font-size: 13px;
    line-height: 1.5;
  }

  .input::placeholder {
    color: var(--text-muted);
  }

  .send {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border-radius: 7px;
    background: var(--accent);
    color: #fff;
    transition: background-color 150ms ease;
    flex: none;
  }

  .send:hover:not(:disabled) {
    background: color-mix(in srgb, var(--accent) 88%, #000);
  }

  .send:disabled {
    background: var(--border);
    color: var(--text-muted);
  }
</style>
