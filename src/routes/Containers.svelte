<script>
import { containers } from "../stores/containers";
import Container from "../lib/Container.svelte";

</script>
{#each $containers as container}
    <Container {...container} />
{/each}