import { invoke } from "@tauri-apps/api"

export const docker = {
    container: {
        start: async (containerId: string) => invoke("start_container", {
            containerId
        }),
        stop: async (containerId: string) => invoke("stop_container", {
            containerId
        }),
        delete: async (containerId: string) => invoke("delete_container", {
            containerId
        }),
        restart: async (containerId: string) => invoke("restart_container", {
            containerId
        }),
        logs: async (containerId: string) => invoke("container_logs", {
            containerId
        })
    }
}