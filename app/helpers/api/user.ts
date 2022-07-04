import { invoke } from '@tauri-apps/api/tauri'

interface User {
    id: string;
    username: string;
    email: string;
    token?: string;
    git_token?: string;
}

export const create = async (obj: { username: string; email: string; password: string; }) => {
    return await invoke('user_create', obj)
}