export enum MixnetNodeMsgType {
    REGISTER = "register",
    ENCRYPTION_RESULT = "encryption_result",
    DECRYPTION_RESULT = "decryption_result",
    KEY_RESPONSE = "key_response",
}

export interface MixnetNodeMsg {
    type: MixnetNodeMsgType;
    data: any;
    callback?: string;
}
