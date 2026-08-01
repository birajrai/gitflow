<script lang="ts">
  import ChatMessage from './ChatMessage.svelte';
  import type { ChatMessage as Message } from '../lib/chat';

  let {
    messages,
    hasRepo,
    busy,
    onCommitCard,
  }: {
    messages: Message[];
    hasRepo: boolean;
    busy: boolean;
    onCommitCard: (id: number, edited: boolean) => void;
  } = $props();

  let scroller: HTMLDivElement;
  let stickToBottom = true;

  function onScroll(): void {
    if (!scroller) return;
    stickToBottom =
      scroller.scrollHeight - scroller.scrollTop - scroller.clientHeight < 48;
  }

  $effect(() => {
    messages.length;
    if (stickToBottom && scroller) {
      scroller.scrollTop = scroller.scrollHeight;
    }
  });
</script>

<div
  class="window"
  bind:this={scroller}
  onscroll={onScroll}
>
  {#if messages.length === 0}
    <div class="welcome">
      {#if hasRepo}
        <h2>Ask anything or give a command</h2>
        <p>Try "what changed?", "commit and push", or "undo last commit".</p>
      {:else}
        <h2>Open a git repository to start</h2>
        <p>Pick a folder from the top bar. Changes appear here live.</p>
      {/if}
    </div>
  {:else}
    {#each messages as message (message.id)}
      <ChatMessage message={message} busy={busy} onCommitCard={onCommitCard} />
    {/each}
  {/if}
</div>

<style>
  .window {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 16px 18px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .welcome {
    margin: auto;
    max-width: 380px;
    text-align: center;
    color: var(--text-muted);
  }

  .welcome h2 {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-secondary);
    margin: 0 0 6px;
  }

  .welcome p {
    margin: 0;
    font-size: 12px;
  }
</style>
