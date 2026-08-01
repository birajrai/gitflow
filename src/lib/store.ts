import { Store } from '@tauri-apps/plugin-store';

export const DEFAULT_MODEL = 'meta/llama-3.3-70b-instruct';

export interface Settings {
  apiKey: string;
  model: string;
  lastRepo: string;
}

let storePromise: Promise<Store> | null = null;

function getStore(): Promise<Store> {
  storePromise ??= Store.load('settings.json');
  return storePromise;
}

export async function loadSettings(): Promise<Settings> {
  const store = await getStore();
  const apiKey = ((await store.get<string>('apiKey')) as string | null) ?? '';
  const model =
    ((await store.get<string>('model')) as string | null) ?? DEFAULT_MODEL;
  const lastRepo = ((await store.get<string>('lastRepo')) as string | null) ?? '';
  return { apiKey, model, lastRepo };
}

export async function saveSettings(patch: Partial<Settings>): Promise<void> {
  const store = await getStore();
  for (const [key, value] of Object.entries(patch)) {
    await store.set(key, value);
  }
  await store.save();
}
