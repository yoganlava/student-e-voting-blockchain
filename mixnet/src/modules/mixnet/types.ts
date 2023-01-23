export enum MixnetNodeMsgType {
    REGISTER = "register",
    ENCRYPTION_RESULT = "encryption_result",
    DECRYPTION_RESULT = "decryption_result",
}

interface RegisterMsg {
    id: string;
}
interface DecryptionResultMsg {
    id: string;
}
interface EncryptionResultMsg {
    id: string;
}

export interface MixnetNodeMsg {
    type: MixnetNodeMsgType;
    data: RegisterMsg | DecryptionResultMsg | EncryptionResultMsg;
}
