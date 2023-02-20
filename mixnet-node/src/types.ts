import { encrypt } from "eciesjs";

export enum MixnetMessageType {
    // create key pair for given poll_id
    CREATE_KEY = "create_key",
    DECRYPT = "decrypt",
    // ? No need for encrypt as gateway (or user? haven't decided) will have public key to encrypt with
    ENCRYPT = "encrypt",
}

export interface MixnetMessage {
    type: MixnetMessageType;
    data: any;
    callback?: string;
}
