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
                ? container.Names.some((name) => includes(name, search))
                : true;
        })
        .reduce<UIContainers>(
            (prev: UIContainers, curr: Container) => {
                const project =
                curr?.Labels && "com.docker.compose.project" in curr?.Labels
                        ? curr.Labels["com.docker.compose.project"]
                        : false;

                if (project) {
                    if (!(project in prev.compose)) {
                        prev.compose[project] = [];
                    }

                    prev.compose[project].push(curr);
                } else {
                    prev.singles.push(curr);
                }
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
<div class="containers">
    {#if $state.containers.length !== 0}
        <div class="section">
            {#each filteredContainers.singles as container}
                <ContainerComponent {container} />
            {/each}
        </div>
        {#each Object.keys(filteredContainers.compose) as compose}
            <div class="section">
                <h2>{compose}</h2>
                {#each filteredContainers.compose[compose] as service}
                    <ContainerComponent container={service} />
                {/each}
            </div>
        {/each}
    {:else}
        no containers
    {/if}
    </div>

<style lang="scss">
    .containers {
        width: 100%;

        .section {
            width: 100%;
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
