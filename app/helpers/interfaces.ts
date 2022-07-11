export interface User {
    id: string;
    username: string;
    email: string;
    password: string;
    created_at: string;
    git_token?: string;
}