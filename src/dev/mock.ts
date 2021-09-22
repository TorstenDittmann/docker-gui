import type { StateStore } from "src/stores/state";

export const mockState: StateStore = {
    containers: [
        {
            id: "1",
            names: ["appwrite-appwrite"],
            state: "running",
            status: "unknwon",
            image: "appwrite/appwrite:1.0.0",
            labels: {
                "a": "label-1",
                "b": "label-2",
                "c": "label-3",
            },

        }
    ]
}