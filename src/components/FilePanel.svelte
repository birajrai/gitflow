<script lang="ts">
  import type { FileStatus } from '../lib/git';

  let { statuses, hasRepo }: { statuses: FileStatus[]; hasRepo: boolean } = $props();
</script>

<aside class="panel">
  <div class="head">
    <span class="title">Changed files</span>
    {#if statuses.length > 0}
      <span class="count mono">{statuses.length}</span>
    {/if}
  </div>

  {#if !hasRepo}
    <div class="empty">
      <p>Open a folder to see changes.</p>
    </div>
  {:else if statuses.length === 0}
    <div class="empty">
      <p>Nothing to commit.</p>
    </div>
  {:else}
    <ul class="list">
      {#each statuses as file (file.path)}
        <li class="item" title={file.path}>
          <span class="st mono">{file.status}</span>
          <span class="path mono">{file.path}</span>
        </li>
      {/each}
    </ul>
  {/if}
</aside>

<style>
  .panel {
    display: flex;
    flex-direction: column;
    min-height: 0;
    border-right: 1px solid var(--border-subtle);
    background: var(--bg-base);
  }

  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 34px;
    padding: 0 12px;
    flex: none;
  }

  .title {
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-muted);
  }

  .count {
    color: var(--text-secondary);
  }

  .list {
    list-style: none;
    margin: 0;
    padding: 0 6px 12px;
    overflow-y: auto;
    min-height: 0;
  }

  .item {
    display: flex;
    align-items: baseline;
    gap: 8px;
    padding: 4px 6px;
    border-radius: 5px;
    cursor: default;
  }

  .item:hover {
    background: var(--bg-hover);
  }

  .st {
    color: var(--text-secondary);
    width: 18px;
    flex: none;
    text-align: center;
    background: var(--bg-hover);
    border-radius: 3px;
  }

  .path {
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
  }

  .empty {
    padding: 24px 16px;
    color: var(--text-muted);
    font-size: 12px;
    text-align: center;
  }

  .empty p {
    margin: 0;
  }
</style>
