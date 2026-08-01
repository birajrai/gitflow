<script lang="ts">
  import { onMount } from 'svelte';

  let {
    text,
    committed,
    edited,
    busy,
    onEdit,
    onCommit,
  }: {
    text: string;
    committed: boolean;
    edited: boolean;
    busy: boolean;
    onEdit: (text: string) => void;
    onCommit: (edited: boolean) => void;
  } = $props();

  let value = $state('');
  let textarea: HTMLTextAreaElement;

  onMount(() => {
    value = text;
    autogrow();
  });

  function handleInput(): void {
    onEdit(value);
    autogrow();
  }

  function autogrow(): void {
    if (!textarea) return;
    textarea.style.height = 'auto';
    textarea.style.height = textarea.scrollHeight + 'px';
  }
</script>

<div class="card" class:done={committed}>
  <textarea
    class="mono"
    bind:this={textarea}
    bind:value
    rows="2"
    disabled={committed}
    oninput={handleInput}
    aria-label="Commit message"
  ></textarea>

  <div class="foot">
    <div class="meta">
      {#if committed}
        <span class="done-badge">
          <span class="check" aria-hidden="true">&#10003;</span> Committed
        </span>
      {:else if edited}
        <span class="edited-tag">edited</span>
      {/if}
    </div>
    <div class="btns">
      <button class="btn primary" disabled={busy || committed} onclick={() => onCommit(false)}>
        {#if busy}<span class="spinner"></span>{/if}
        Commit
      </button>
      <button class="btn" disabled={busy || committed} onclick={() => onCommit(true)}>
        Edit &amp; Commit
      </button>
    </div>
  </div>
</div>

<style>
  .card {
    border: 1px solid var(--border);
    border-left: 3px solid var(--accent);
    border-radius: 8px;
    background: var(--bg-surface);
    padding: 10px;
    width: min(560px, 100%);
    display: flex;
    flex-direction: column;
    gap: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
  }

  .card.done {
    border-left-color: var(--success);
    opacity: 0.85;
  }

  textarea {
    width: 100%;
    resize: none;
    border: 1px solid var(--border-subtle);
    background: var(--bg-base);
    font-size: 13px;
    line-height: 1.5;
    overflow: hidden;
  }

  .foot {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .meta {
    font-size: 12px;
  }

  .done-badge {
    color: var(--success);
    font-weight: 500;
  }

  .check {
    font-weight: 700;
  }

  .edited-tag {
    color: var(--text-muted);
    font-style: italic;
  }

  .btns {
    display: flex;
    gap: 6px;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 5px 12px;
    border-radius: 6px;
    font-size: 13px;
    border: 1px solid var(--border);
    color: var(--text-primary);
    transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
  }

  .btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent);
    color: var(--accent);
  }

  .btn.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }

  .btn.primary:hover {
    background: color-mix(in srgb, var(--accent) 88%, #000);
    color: #fff;
  }

  .btn:disabled .spinner {
    color: #fff;
  }
</style>
