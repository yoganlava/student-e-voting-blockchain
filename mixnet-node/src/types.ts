export enum MixnetMessageType {
    // Key is given as part of distribution
    KEY,
    DECRYPT,
    ENCRYPT
}

export interface MixnetMessage {
    type: MixnetMessageType,
    data: any
}