import {Writable, writable} from "svelte/store";

export interface DragMessage {
    targetZoneRef: string,
    dropMessage: string,
}

export const dragMessage = writable<DragMessage>(
    {
        targetZoneRef: "",
        dropMessage: "",
    }
);