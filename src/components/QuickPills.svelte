<script lang="ts">
  let {
    hasChanges,
    busy,
    onCommit,
    onPush,
    onCommitPush,
  }: {
    hasChanges: boolean;
    busy: boolean;
    onCommit: () => void;
    onPush: () => void;
    onCommitPush: () => void;
  } = $props();
</script>

<div class="pills">
  <button class="pill" disabled={!hasChanges || busy} onclick={onCommit} title="Stage all and commit with an AI-generated message">
    Commit
  </button>
  <button class="pill" disabled={!hasChanges || busy} onclick={onPush} title="Push current branch">
    Push
  </button>
  <button class="pill" disabled={!hasChanges || busy} onclick={onCommitPush} title="Commit then push">
    Commit &amp; Push
  </button>
</div>

<style>
  .pills {
    display: flex;
    align-items: center;
    gap: 8px;
    height: var(--pills-h);
    padding: 0 18px;
    border-top: 1px solid var(--border-subtle);
    flex: none;
  }

  .pill {
    padding: 4px 14px;
    border-radius: 999px;
    border: 1px solid var(--border);
    background: var(--bg-surface);
    font-size: 12.5px;
    color: var(--text-secondary);
    transition: color 150ms ease, border-color 150ms ease, background-color 150ms ease;
  }

  .pill:not(:disabled):hover {
    color: var(--accent);
    border-color: var(--accent);
    background: var(--accent-subtle);
  }

  .pill:disabled {
    opacity: 0.4;
    color: var(--text-muted);
  }
</style>
