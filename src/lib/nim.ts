import { invoke } from '@tauri-apps/api/core';

export const generateCommitMessage = (apiKey: string, model: string, diff: string): Promise<string> =>
  invoke('generate_commit_message', { apiKey, model, diff });

export const testNimConnection = (apiKey: string, model: string): Promise<void> =>
  invoke('test_nim_connection', { apiKey, model });
