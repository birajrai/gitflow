<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';

  import TopBar from '../components/TopBar.svelte';
  import FilePanel from '../components/FilePanel.svelte';
  import ChatWindow from '../components/ChatWindow.svelte';
  import QuickPills from '../components/QuickPills.svelte';
  import ChatInput from '../components/ChatInput.svelte';
  import SettingsPanel from '../components/SettingsPanel.svelte';

  import {
    messages,
    addUser,
    addAssistant,
    addStatus,
    addError,
    addCommitCard,
    getCommitCard,
    updateCommitCard,
  } from '../lib/chat';
  import {
    openRepo,
    getDiff,
    stageAll,
    commit as gitCommit,
    push as gitPush,
    resetSoft,
    type RepoInfo,
  } from '../lib/git';
  import { generateCommitMessage } from '../lib/nim';
  import { loadSettings, saveSettings, type Settings } from '../lib/store';

  let repo = $state<RepoInfo | null>(null);
  let repoPath = $state<string | null>(null);
  let settings = $state<Settings>({ apiKey: '', model: '', lastRepo: '' });
  let busy = $state(false);
  let settingsOpen = $state(false);
  let timer: ReturnType<typeof setInterval> | undefined;

  function basename(p: string): string {
    return p.split(/[\\/]/).pop() ?? p;
  }

  async function refreshRepo(): Promise<void> {
    if (!repoPath) return;
    try {
      repo = await openRepo(repoPath);
    } catch {
      // background refresh — keep last known state
    }
  }

  async function chooseFolder(): Promise<void> {
    try {
      const picked = await open({ directory: true, multiple: false, title: 'Select a git repository' });
      const path = Array.isArray(picked) ? picked[0] : picked;
      if (!path) return;
      const info = await openRepo(path);
      repo = info;
      repoPath = path;
      await saveSettings({ lastRepo: path });
    } catch (err) {
      addError(String(err));
    }
  }

  async function send(text: string): Promise<void> {
    addUser(text);
    const t = text.trim().toLowerCase();

    if (t === 'commit and push' || t === 'commit & push') return flowCommit(true);
    if (t === 'commit') return flowCommit(false);
    if (t === 'push') return doPush();
    if (t === 'what changed?' || t === "what's changed?" || t === 'what changed' || t === 'status') {
      return summarize();
    }
    if (t === 'undo last commit' || t === 'undo') return undoLast();
    if (t === 'diff') return showDiff();
    if (t.startsWith('commit this as ')) {
      return commitExact(text.slice('commit this as '.length).trim());
    }

    addAssistant(
      'I can help with: `commit`, `commit and push`, `push`, `what changed?`, `undo last commit`, `diff`, or `commit this as <message>`.'
    );
  }

  function requireRepo(): boolean {
    if (!repoPath) {
      addError('Open a git repository first.');
      return false;
    }
    return true;
  }

  async function flowCommit(withPush: boolean): Promise<void> {
    if (busy) return;
    if (!requireRepo()) return;
    if (!repo || repo.is_clean) {
      addStatus('Nothing to commit.');
      return;
    }
    if (!settings.apiKey) {
      addError('No NIM API key set. Open settings (gear icon) and add your key.');
      return;
    }

    busy = true;
    try {
      addStatus('Staging all changes…');
      await stageAll(repoPath!);
      const staged = await openRepo(repoPath!);
      repo = staged;
      addStatus(
        `Staged ${staged.statuses.length} file${staged.statuses.length === 1 ? '' : 's'}. Generating commit message…`
      );
      const diff = await getDiff(repoPath!);
      const msg = await generateCommitMessage(settings.apiKey, settings.model, diff);
      addCommitCard(msg.trim(), withPush);
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function commitCard(id: number, edited: boolean): Promise<void> {
    if (!requireRepo() || busy) return;
    const card = getCommitCard(id);
    if (!card || card.committed) return;

    busy = true;
    try {
      const out = await gitCommit(repoPath!, card.text);
      updateCommitCard(id, { committed: true, edited });
      addStatus(out.trim() || 'Committed.');
      await refreshRepo();
      if (card.pushAfter) {
        await doPush();
      }
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function commitExact(message: string): Promise<void> {
    if (busy) return;
    if (!requireRepo()) return;
    if (!message) {
      addError('Empty commit message.');
      return;
    }
    busy = true;
    try {
      await stageAll(repoPath!);
      const out = await gitCommit(repoPath!, message);
      addStatus(out.trim() || 'Committed.');
      await refreshRepo();
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function doPush(): Promise<void> {
    if (busy) return;
    if (!requireRepo()) return;
    busy = true;
    try {
      addStatus('Pushing…');
      const out = await gitPush(repoPath!);
      addStatus(
        out.trim() ||
          (repo?.branch ? `Pushed to origin/${repo.branch}.` : 'Pushed.')
      );
      await refreshRepo();
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function summarize(): Promise<void> {
    if (!requireRepo()) return;
    if (!repo || repo.is_clean) {
      addStatus('Working tree clean — nothing changed.');
      return;
    }
    const counts: Record<string, number> = {};
    for (const f of repo.statuses) {
      const key =
        f.status[0] === '?'
          ? 'untracked'
          : f.status[0] === 'A'
            ? 'added'
            : f.status[0] === 'D'
              ? 'deleted'
              : f.status[0] === 'M' || f.status[0] === 'R'
                ? 'modified'
                : 'changed';
      counts[key] = (counts[key] ?? 0) + 1;
    }
    const summary = Object.entries(counts)
      .map(([k, v]) => `${v} ${k}`)
      .join(', ');
    const list = repo.statuses.map((f) => `${f.status} ${f.path}`).join('\n');
    addAssistant(
      `${repo.statuses.length} file${repo.statuses.length === 1 ? '' : 's'} changed (${summary}):\n${list}`
    );
  }

  async function undoLast(): Promise<void> {
    if (busy) return;
    if (!requireRepo()) return;
    busy = true;
    try {
      await resetSoft(repoPath!);
      addStatus('Last commit undone (git reset --soft HEAD~1). Changes are staged.');
      await refreshRepo();
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function showDiff(): Promise<void> {
    if (!requireRepo()) return;
    busy = true;
    try {
      const diff = await getDiff(repoPath!);
      if (!diff) {
        addStatus('No diff to show.');
        return;
      }
      const files = diff
        .split('\n')
        .filter((l) => l.startsWith('diff --git'))
        .map((l) => {
          const parts = l.split(' b/');
          return parts.length > 1 ? (parts[parts.length - 1] ?? l) : l;
        });
      addAssistant(
        files.length
          ? `Files changed (${files.length}):\n${files.join('\n')}`
          : 'Diff present, but no file headers found.'
      );
    } catch (err) {
      addError(String(err));
    } finally {
      busy = false;
    }
  }

  async function handleSaved(patch: Partial<Settings>): Promise<void> {
    settings = { ...settings, ...patch };
    await saveSettings(patch);
    settingsOpen = false;
  }

  function tick(): void {
    if (document.hasFocus() && repoPath) {
      void refreshRepo();
    }
  }

  onMount(async () => {
    settings = await loadSettings();
    if (settings.lastRepo) {
      try {
        repo = await openRepo(settings.lastRepo);
        repoPath = settings.lastRepo;
      } catch {
        // stale path from a previous session
      }
    }
    window.addEventListener('focus', tick);
    timer = setInterval(tick, 5000);
  });

  onDestroy(() => {
    window.removeEventListener('focus', tick);
    if (timer) clearInterval(timer);
  });
</script>

<svelte:head>
  <title>GitFlow</title>
</svelte:head>

<div class="app">
  <TopBar
    repoName={repoPath ? basename(repoPath) : ''}
    branch={repo?.branch ?? ''}
    hasRepo={!!repoPath}
    onOpenFolder={chooseFolder}
    onOpenSettings={() => (settingsOpen = true)}
  />

  <div class="main">
    <FilePanel statuses={repo?.statuses ?? []} hasRepo={!!repoPath} />

    <section class="chat-col">
      <ChatWindow
        messages={$messages}
        hasRepo={!!repoPath}
        busy={busy}
        onCommitCard={commitCard}
      />
      <QuickPills
        hasChanges={!!repo && !repo.is_clean}
        busy={busy}
        onCommit={() => flowCommit(false)}
        onPush={doPush}
        onCommitPush={() => flowCommit(true)}
      />
      <ChatInput disabled={busy} onSend={send} />
    </section>
  </div>

  <SettingsPanel
    open={settingsOpen}
    settings={settings}
    busy={busy}
    onClose={() => (settingsOpen = false)}
    onSaved={handleSaved}
  />
</div>

<style>
  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg-base);
  }

  .main {
    flex: 1;
    min-height: 0;
    display: grid;
    grid-template-columns: var(--panel-w) 1fr;
  }

  .chat-col {
    display: flex;
    flex-direction: column;
    min-width: 400px;
    min-height: 0;
    background: var(--bg-base);
  }

  @media (max-width: 760px) {
    .main {
      grid-template-columns: 1fr;
    }

    .main :global(.panel) {
      display: none;
    }
  }
</style>
