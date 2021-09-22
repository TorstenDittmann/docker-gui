import { invoke } from "@tauri-apps/api";
import { mockState } from "../dev/mock";
import { writable } from "svelte/store";
export type Container = {
    id: string;
    names: string[];
    image: string;
    state: string;
    status: string;
    labels: {
        [key: string]: string;
    }
}

export type StateStore = {
    containers: Container[];
}

const initial: StateStore = {
    containers: []
}

const createTauriStore = () => {
    const { subscribe, set, update } = writable<StateStore>(initial);

    return {
        subscribe,
        containers: {
            load: async () => {
                const response = await invoke('containers_list');
                return update(n => {
                    n.containers = JSON.parse(<string>response);
                    return n;
                });
            }
        },
        loadMock: () => set(mockState),
    };
}

export const state = createTauriStore();
