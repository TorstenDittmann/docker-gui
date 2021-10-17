<script lang="ts">
    import { includes } from "../helpers";

    import { state } from "../stores";

    let search = "";

    $: filteredImages = $state.images.filter((image) => {
        return (
            includes(image.Id, search) ||
            image.RepoTags.some((tags) => includes(tags, search))
        );
    });
</script>

<h2>Images</h2>
<table>
    <tbody>
        {#each filteredImages as image}
            <tr>
                <td>{image.RepoTags[0]}</td>
            </tr>
        {:else}
            no containers
        {/each}
    </tbody>
</table>

