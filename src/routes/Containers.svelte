<script>
import { containers } from "../stores/containers";
import Container from "../lib/Container.svelte";

</script>
<table>
    <thead>
        <td>Name</td>
        <td>Image</td>
    </thead>
    <tbody>
    {#each $containers as container}
        <Container {...container} />
    {/each}
    </tbody>
</table>

<style lang="scss">
    table {
        width: 100%;
    }
</style>