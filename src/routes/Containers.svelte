<script lang="ts">
    import { state } from "../stores";
    import Container from "../lib/Container.svelte";
</script>
<table>
    <thead>
        <td>Name</td>
        <td>Image</td>
        <td>Actions</td>
    </thead>
    <tbody>
    {#each $state.containers as container}
        <Container container={container} />
    {/each}
    </tbody>
</table>

<style lang="scss">
    table {
        width: 100%;
    }
</style>