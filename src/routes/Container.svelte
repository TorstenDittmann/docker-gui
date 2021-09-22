<script lang="ts">
    import Fa from "svelte-fa";
    import { faPlay, faStop, faTrash } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";

    import { Container, state } from "../stores/state";

    export let params: {
        id?: string;
    } = {};

    let container: Container;

    onMount(() => {
        container = $state.containers.find((c) => c.id === params.id);
    });
</script>

<div class="container">
    {#if container}
        <h1>{container.names[0]}</h1>
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
</style>