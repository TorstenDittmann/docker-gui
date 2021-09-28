<script lang="ts">
    import Header from "../lib/Header.svelte";
    import { onMount } from "svelte";
    import { pop } from "svelte-spa-router";

    import { Container, state } from "../stores/state";
import Fa from "svelte-fa";
import { faArrowLeft, faPlay, faStop, faTrash } from "@fortawesome/free-solid-svg-icons";

    export let params: {
        id?: string;
    } = {};

    let container: Container;

    onMount(() => {
        container = $state.containers.find((c) => c.id === params.id);
    });
</script>

<Header>
    <span class="action back" on:click={pop}><Fa icon={faArrowLeft} /></span>
    {#if container}
        <h1>{container.names[0]}</h1>
    {/if}
</Header>
<div class="container">
    {#if container}
        <h2>{container.image}</h2>
        <p>id: {container.id}</p>
        <p>state: {container.state}</p>
        <p>status: {container.status}</p>
        <p>labels:</p>
        <table>
            {#each Object.keys(container.labels) as key}
                <tr>
                    <td>{key}</td>
                    <td>{container.labels[key]}</td>
                </tr>
            {/each}
        </table>
    {:else}
        loading
    {/if}
</div>

<style lang="scss">
    .container {
        text-align: left;
    }

    h1 {
        margin-right: 1.5rem;
    }

    .action {
        margin-right: .25rem;
        margin-left: .25rem;
        cursor: pointer;

        &.back {
            margin-right: 1.25rem;
        } 
        &:hover {
            color: green;
        }
    }
</style>