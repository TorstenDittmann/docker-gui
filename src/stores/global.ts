import { readable } from "svelte/store";
export type Global = {
    inTauri: boolean;
}

export const global = readable<Global>({
    inTauri: "__TAURI__" in window
});
