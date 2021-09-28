<script lang="ts">
    import { includes } from "../helpers";

    import Header from "../lib/Header.svelte";
    import { state } from "../stores";

    let search = "";

    $: filteredImages = $state.images.filter((image) => {
        return (
            includes(image.id, search) ||
            image.repo_tags.some((tags) => includes(tags, search))
        );
    });
</script>

<Header>
    <input bind:value={search} placeholder="Search..." autofocus />
</Header>
<h2>Images</h2>
<table>
    <tbody>
        {#each filteredImages as image}
            <tr>
                <td>{image.repo_tags[0]}</td>
            </tr>
        {:else}
            no containers
        {/each}
    </tbody>
</table>

<style lang="scss">
    input {
        line-height: 4rem;
        font-size: 1.5rem;
        outline: 0;
        border: none;
        width: 100%;
    }
</style>
