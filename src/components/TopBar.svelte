<script lang="ts">
  let {
    repoName,
    branch,
    hasRepo,
    onOpenFolder,
    onOpenSettings,
  }: {
    repoName: string;
    branch: string;
    hasRepo: boolean;
    onOpenFolder: () => void;
    onOpenSettings: () => void;
  } = $props();
</script>

<header class="topbar">
  <div class="repo">
    {#if hasRepo}
      <span class="dot" aria-hidden="true"></span>
      <span class="branch mono">&#9098; {branch}</span>
      <span class="name" title={repoName}>{repoName}</span>
    {:else}
      <span class="name muted">No repository</span>
    {/if}
  </div>
  <div class="actions">
    <button class="icon-btn" onclick={onOpenFolder} title="Open repository folder" aria-label="Open folder">
      <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round">
        <path d="M1.5 3.5h4.2l1.5 2H14.5A.5.5 0 0 1 15 6v6.5a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5V4a.5.5 0 0 1 .5-.5Z" />
        <path d="M1.5 6.5h13.5" />
      </svg>
    </button>
    <button class="icon-btn" onclick={onOpenSettings} title="Settings" aria-label="Settings">
      <svg viewBox="0 0 16 16" width="15" height="15" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round">
        <circle cx="8" cy="8" r="2.2" />
        <path d="M8 1.8v2M8 12.2v2M1.8 8h2M12.2 8h2M3.6 3.6l1.4 1.4M11 11l1.4 1.4M12.4 3.6 11 5M5 11l-1.4 1.4" />
      </svg>
    </button>
  </div>
</header>

<style>
  .topbar {
    height: var(--topbar-h);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    background: var(--bg-surface);
    border-bottom: 1px solid var(--border-subtle);
  }

  .repo {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex: none;
  }

  .branch {
    color: var(--text-secondary);
    background: var(--bg-hover);
    border: 1px solid var(--border-subtle);
    border-radius: 4px;
    padding: 2px 6px;
    white-space: nowrap;
  }

  .name {
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .muted {
    color: var(--text-muted);
    font-weight: 400;
  }

  .actions {
    display: flex;
    gap: 4px;
  }

  .icon-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    color: var(--text-secondary);
    transition: color 150ms ease, background-color 150ms ease;
  }

  .icon-btn:hover {
    color: var(--accent);
    background: var(--bg-hover);
  }
</style>
