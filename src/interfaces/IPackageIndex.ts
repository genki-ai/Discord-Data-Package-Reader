import { IAccount } from "./IAccount";
import { IChannel } from "./IChannel";

export interface IPackageIndex {
    account: IAccount
    channal_list: Array<IChannel> | null
}