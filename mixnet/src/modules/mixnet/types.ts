export enum MixnetNodeMsgType {
    REGISTER = "register",
    RESULT = "result"
}

export interface MixnetNodeMsg {
    type: MixnetNodeMsgType
}