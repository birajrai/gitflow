import { writable, type Writable } from 'svelte/store';

export type ChatMessage =
  | { kind: 'user'; id: number; text: string }
  | { kind: 'assistant'; id: number; text: string }
  | {
      kind: 'commit';
      id: number;
      text: string;
      committed: boolean;
      pushAfter: boolean;
      edited: boolean;
    }
  | { kind: 'status'; id: number; text: string }
  | { kind: 'error'; id: number; text: string };

export type NewMessage =
  | { kind: 'user'; text: string }
  | { kind: 'assistant'; text: string }
  | { kind: 'commit'; text: string; committed: boolean; pushAfter: boolean; edited: boolean }
  | { kind: 'status'; text: string }
  | { kind: 'error'; text: string };

export type CommitCardMessage = Extract<ChatMessage, { kind: 'commit' }>;

let nextId = 1;

export const messages: Writable<ChatMessage[]> = writable([]);

function push(msg: NewMessage & { id?: number }): number {
  const id = msg.id ?? nextId++;
  messages.update((list) => [...list, { ...msg, id }] as ChatMessage[]);
  return id;
}

export function addUser(text: string): number {
  return push({ kind: 'user', text });
}

export function addAssistant(text: string): number {
  return push({ kind: 'assistant', text });
}

export function addCommitCard(text: string, pushAfter: boolean): number {
  return push({ kind: 'commit', text, committed: false, pushAfter, edited: false });
}

export function addStatus(text: string): number {
  return push({ kind: 'status', text });
}

export function addError(text: string): number {
  return push({ kind: 'error', text });
}

export function updateCommitCard(
  id: number,
  patch: Partial<Pick<CommitCardMessage, 'text' | 'committed' | 'edited'>>
): void {
  messages.update((list) =>
    list.map((m) => (m.kind === 'commit' && m.id === id ? { ...m, ...patch } : m))
  );
}

export function getCommitCard(id: number): CommitCardMessage | undefined {
  let found: CommitCardMessage | undefined;
  messages.update((list) => {
    found = list.find((m): m is CommitCardMessage => m.kind === 'commit' && m.id === id);
    return list;
  });
  return found;
}
