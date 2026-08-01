<script lang="ts">
  import CommitCard from './CommitCard.svelte';
  import { updateCommitCard, type ChatMessage as Message } from '../lib/chat';

  let {
    message,
    busy,
    onCommitCard,
  }: {
    message: Message;
    busy: boolean;
    onCommitCard: (id: number, edited: boolean) => void;
  } = $props();

  function updateText(id: number, text: string): void {
    updateCommitCard(id, { text });
  }
</script>

{#if message.kind === 'user'}
  <div class="row user">
    <div class="bubble">{message.text}</div>
  </div>
{:else if message.kind === 'assistant'}
  <div class="row assistant">
    <div class="text">{message.text}</div>
  </div>
{:else if message.kind === 'commit'}
  <div class="row assistant">
    <CommitCard
      text={message.text}
      committed={message.committed}
      edited={message.edited}
      busy={busy}
      onEdit={(text) => updateText(message.id, text)}
      onCommit={(edited) => onCommitCard(message.id, edited)}
    />
  </div>
{:else if message.kind === 'status'}
  <div class="row assistant">
    <div class="text status">
      <span class="check" aria-hidden="true">&#10003;</span>
      {message.text}
    </div>
  </div>
{:else if message.kind === 'error'}
  <div class="row assistant">
    <div class="text error">{message.text}</div>
  </div>
{/if}

<style>
  .row {
    display: flex;
  }

  .row.user {
    justify-content: flex-end;
  }

  .bubble {
    background: var(--chat-user-bg);
    color: #fff;
    border-radius: 12px 12px 3px 12px;
    padding: 8px 12px;
    max-width: 72%;
    font-size: 14px;
    line-height: 1.45;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .row.assistant {
    justify-content: flex-start;
  }

  .text {
    font-size: 14px;
    line-height: 1.5;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-word;
    max-width: 92%;
  }

  .text.status {
    color: var(--text-secondary);
    font-size: 13px;
    display: flex;
    align-items: baseline;
    gap: 6px;
  }

  .check {
    color: var(--success);
    font-weight: 700;
  }

  .text.error {
    border-left: 2px solid var(--error);
    background: color-mix(in srgb, var(--error) 8%, transparent);
    padding: 8px 12px;
    border-radius: 0 6px 6px 0;
    color: var(--error);
    font-size: 13px;
  }
</style>
