export interface Response {
    message: string;
}

export interface InfoResponse {
    projects: number;
    users: number;
}

export interface UserResponse {
    id: string;
    username: string;
    email: string;
    created_at: string;
    token: string;
}