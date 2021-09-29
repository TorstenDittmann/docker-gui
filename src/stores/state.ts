import { event, invoke } from "@tauri-apps/api";
import { mockState } from "../dev/mock";
import { writable } from "svelte/store";

export type Container = {
    Created: number;
    Command: string;
    Id: string;
    Names: string[];
    Image: string;
    State: string;
    Status: string;
    Labels: {
        [key: string]: string;
    },
    Ports: {
        Ip: string;
        PrivatePort: number;
        PublicPort: number;
        Type: string;
    }[]
}

export type Image = {
    Created: number;
    Id: string;
    ParentId: string;
    Labels: string[];
    RepoTags: string[];
    RepoDigests: string[];
    VirtualSize: string[];
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
                    n.containers = JSON.parse(<string>response);
                    return n;
                });
            },
            update: (container: Partial<Container>) => update(n => {
                const index = n.containers.findIndex(c => c.Id === container.Id);
                n.containers[index] = {
                    ...n.containers[index],
                    ...container
                }
                return n;
            }),
            remove: (id: string) => update(n => {
                n.containers = n.containers.filter(c => c.Id !== id);
                return n;
            })
        },
        images: {
            load: async () => {
                const response = await invoke('images_list');
                console.log(JSON.parse(<string>response));
                return update(n => {
                    n.images = JSON.parse(<string>response);
                    return n;
                })
            }
        },
        loadMock: () => set(mockState),
    };
}

export const state = createTauriStore();

type DockerEvent = {
    Action: string;
    id: string;
    status: string;
};

event.listen<DockerEvent>('docker', async ({ payload }) => {
    switch (payload.Action) {
        case "stop":
        case "kill":
        case "die":
            console.log(payload.status)
            state.containers.update({
                Id: payload.id,
                State: payload.status
            })
            break;

        case "start":
        case "create":
            await state.containers.load();
            break;

        case "destroy":
            state.containers.remove(payload.id);
            break;

        default:
            console.log(payload)
            break;
    }
});

