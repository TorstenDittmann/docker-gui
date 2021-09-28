<script lang="ts">
    import type { Container } from "../stores/state";
    import { state } from "../stores";
    import ContainerComponent from "../lib/Container.svelte";
    import Header from "../lib/Header.svelte";
    import { includes } from "../helpers";

    type UIContainers = {
        compose: {
            [key: string]: Container[];
        };
        singles: Container[];
    };

    let search = "";

    $: filteredContainers = $state.containers
        .filter((container) => {
            return search !== ""
                ? container.names.some((name) => includes(name, search))
                : true;
        })
        .reduce<UIContainers>(
            (prev: UIContainers, curr: Container) => {
                const project =
                    curr.labels["com.docker.compose.project"] ?? false;
                if (project) {
                    if (!(project in prev.compose)) {
                        prev.compose[project] = [];
                    }

                    prev.compose[project].push(curr);
                } else {
                    prev.singles.push(curr);
                }
                console.log(prev);
                return prev;
            },
            {
                compose: {},
                singles: [],
            }
        );
</script>

<Header>
    <input bind:value={search} placeholder="Search..." />
</Header>
<table>
    {#if $state.containers.length !== 0}
        {#each Object.keys(filteredContainers.compose) as compose}
            <tbody>
                <h2>{compose}</h2>
                {#each filteredContainers.compose[compose] as service}
                    <ContainerComponent container={service} />
                {/each}
            </tbody>
        {/each}
        <tbody>
            {#each filteredContainers.singles as container}
                <ContainerComponent {container} />
            {/each}
        </tbody>
    {:else}
        no containers
    {/if}
</table>

<style lang="scss">
    table {
        width: 100%;

        tbody {
            h2 {
                margin: 0;
                line-height: 3rem;
            }
        }
    }

    input {
        line-height: 4rem;
        font-size: 1.5rem;
        outline: 0;
        border: none;
        width: 100%;
    }
</style>
