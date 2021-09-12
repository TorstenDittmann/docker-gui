import { writable } from "svelte/store";
export type Container = {
    id: string;
    names: string[];
    image: string;
    state: string;
    status: string;
}
export const containers = writable<Container[]>([]);
