import { invoke } from "@tauri-apps/api";
import { mockState } from "../dev/mock";
import { writable } from "svelte/store";

export type Container = {
    created: number;
    command: string;
    id: string;
    names: string[];
    image: string;
    state: string;
    status: string;
    labels: {
        [key: string]: string;
    },
    ports: {
        ip: string;
        private_port: number;
        public_port: number;
        typ: string;
   }[] 
}

export type Image = {
    created: number;
    id: string;
    parent_id: string;
    labels: string[];
    repo_tags: string[];
    repo_digests: string[];
    virtual_size: string[];
}

export type StateStore = {
    containers: Container[];
    images: Image[];
}

const initial: StateStore = {
    containers: [],
    images: []
}

const createTauriStore = () => {
    const { subscribe, set, update } = writable<StateStore>(initial);

    return {
        subscribe,
        containers: {
            load: async () => {
                const response = await invoke('containers_list');
                return update(n => {
                    n.containers = JSON.parse(<string> response);
                    console.log(n.containers)
                    return n;
                });
            }
        },
        images: {
            load: async () => {
                const response = await invoke('images_list');
                return update(n => {
                    n.images = JSON.parse(<string> response);
                    return n;
                })
            }
        },
        loadMock: () => set(mockState),
    };
}

export const state = createTauriStore();
