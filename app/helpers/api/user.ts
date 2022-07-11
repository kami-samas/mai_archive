import { invoke } from '@tauri-apps/api/tauri'
import { UserResponse } from './types';


export const create = async (obj: { username: string; email: string; password: string; }) => {
    return await invoke('user_create', obj) as UserResponse
}

export const login = async (obj: { email: string; password: string; }) => {
    return await invoke('user_login', obj) as UserResponse
}