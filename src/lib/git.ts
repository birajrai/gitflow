import { invoke } from '@tauri-apps/api/core';

export interface FileStatus {
  status: string;
  path: string;
}

export interface RepoInfo {
  branch: string;
  statuses: FileStatus[];
  is_clean: boolean;
}

export const openRepo = (path: string): Promise<RepoInfo> => invoke('open_repo', { path });

export const getStatus = (path: string): Promise<FileStatus[]> => invoke('get_status', { path });

export const getDiff = (path: string): Promise<string> => invoke('get_diff', { path });

export const stageAll = (path: string): Promise<void> => invoke('stage_all', { path });

export const commit = (path: string, message: string): Promise<string> =>
  invoke('commit', { path, message });

export const push = (path: string): Promise<string> => invoke('push', { path });

export const resetSoft = (path: string): Promise<void> => invoke('reset_soft', { path });
