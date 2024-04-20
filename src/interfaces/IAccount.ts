export interface IAccount {
    id: string;
    username: string;
    discriminator: number;
    global_name: string;
    email: string;
    verified: boolean;
    has_mobile: boolean;
    phone: string;
    ip: string;
}