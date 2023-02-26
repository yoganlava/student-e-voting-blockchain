import { writable, type Writable } from "svelte/store";

class VoterStore {
    isAdmin: Writable<boolean> = writable(false);
    voterInfo: Writable<Object> = writable(undefined);
}

export const voterStore = new VoterStore();