export * from './user'
export * from './types'

import { invoke } from '@tauri-apps/api/tauri'
import { Response, InfoResponse } from './types'


export const get_am_pm = async () => {
    return await invoke('get_am_pm') as Response;
} 

export const get_info = async () => {
    return await invoke('get_info') as InfoResponse;
}