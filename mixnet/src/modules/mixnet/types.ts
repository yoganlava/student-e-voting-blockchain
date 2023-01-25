export enum MixnetNodeMsgType {
    REGISTER = "register",
    ENCRYPTION_RESULT = "encryption_result",
    DECRYPTION_RESULT = "decryption_result",
}

export interface MixnetNodeMsg {
    type: MixnetNodeMsgType;
    data: any;
}