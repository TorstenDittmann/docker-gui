import type { StateStore } from "src/stores/state";

export const mockState: StateStore = {
    images: [],
    containers: [
        {
            Id: "1",
            Names: ["appwrite-appwrite"],
            State: "running",
            Status: "unknwon",
            Image: "appwrite/appwrite:1.0.0",
            Command: "hello world",
            Created: 123456789,
            Ports: [
                {
                    Ip: "0.0.0.0",
                    PrivatePort: 80,
                    PublicPort: 80,
                    Type: "tcp"
                }
            ],
            Labels: {
                "a": "label-1",
                "b": "label-2",
                "c": "label-3",
            },

        }
    ]
}