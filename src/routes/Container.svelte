<script lang="ts">
    import Header from "../lib/Header.svelte";
    import { onMount } from "svelte";
    import { pop } from "svelte-spa-router";

    import { Container, state } from "../stores/state";
    import Fa from "svelte-fa";
    import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
    import { docker } from "../docker";
    import { event } from "@tauri-apps/api";

    export let params: {
        id?: string;
    } = {};

    let container: Container;
    let containerElement: HTMLDivElement;

    let logs = [];

    onMount(() => {
        container = $state.containers.find((c) => c.id === params.id);
        docker.container.logs(params.id);

        event.listen<string>("logs", (event) => {
            if (event.event === "logs") {
                logs = [...logs, event.payload];
                containerElement.scrollIntoView(false);
            }
        });
    });
</script>

<Header>
    <span class="action back" on:click={pop}><Fa icon={faArrowLeft} /></span>
    {#if container}
        <h1>{container.names[0]}</h1>
        <h1>{container.image}</h1>
    {/if}
</Header>
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

    h1 {
        margin-right: 1.5rem;
    }

    .action {
        margin-right: 0.25rem;
        margin-left: 0.25rem;
        cursor: pointer;

        &.back {
            margin-right: 1.25rem;
        }
        &:hover {
            color: green;
        }
    }
</style>
