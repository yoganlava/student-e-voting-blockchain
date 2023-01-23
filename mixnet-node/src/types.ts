export enum MixnetMessageType {
    // Key is given as part of distribution
    KEY,
    DECRYPT,
    ENCRYPT
}

interface KeyMsg {
    key: string,
    poll_id: number
}

interface DecryptMsg {
    poll_id: number,
    votes: Array<any>
}

export interface MixnetMessage {
    type: MixnetMessageType,
    data: KeyMsg
}