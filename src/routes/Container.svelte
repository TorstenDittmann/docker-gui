<script lang="ts">
    import { onMount } from "svelte";
    import { Container, state } from "../stores/state";
    import { docker } from "../docker";
    import { event } from "@tauri-apps/api";

    export let params: {
        id?: string;
    } = {};

    let container: Container;
    let containerElement: HTMLDivElement;

    let logs = [];

    onMount(() => {
        console.log(params)
        container = $state.containers.find((c) => c.Id === params.id);
        docker.container.logs(params.id);

        event.listen<string>("logs", (event) => {
            if (event.event === "logs") {
                console.log(event);
                logs = [...logs, event.payload];
                containerElement?.scrollIntoView(false);
            }
        });
    });
</script>

<div class="logs" bind:this={containerElement}>
    {#each logs as log}
        <p>{log}</p>
    {/each}
</div>

<style lang="scss">
    .logs {
        text-align: left;

        p {
            font-size: 0.75rem;
            margin: 0;
            padding: 0;
        }
    }
</style>
