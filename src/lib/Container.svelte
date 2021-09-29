<script lang="ts">
    import Fa from "svelte-fa";
    import {
        faPlay,
        faServer,
        faStop,
        faSync,
        faTrash,
    } from "@fortawesome/free-solid-svg-icons";
    import { push } from "svelte-spa-router";

    import type { Container } from "../stores/state";
    import { docker } from "../docker";

    export let container: Container;
</script>

<div class="container">
    <span on:click={() => push(`/container/${container.Id}`)}>
        <span
            class="state"
            class:running={container.State === "running"}
            class:stopped={container.State === "exited"}
            class:starting={['kill', 'die'].includes(container.State)}
        >
            <Fa icon={faServer} />
        </span>
        {container.Names[0]}
    </span>
    <span class="action">
        <span on:click={() => docker.container.start(container.Id)}>
            <Fa icon={faPlay} />
        </span>
        <span on:click={() => docker.container.stop(container.Id)}>
            <Fa icon={faStop} />
        </span>
        <span on:click={() => docker.container.restart(container.Id)}>
            <Fa icon={faSync} />
        </span>
        <span on:click={() => docker.container.delete(container.Id)}>
            <Fa icon={faTrash} />
        </span>
    </span>
</div>

<style lang="scss">
    .container {
        line-height: 3rem;
        cursor: pointer;
        width: 100%;
        display: flex;
        justify-content: space-between;

        .state {
            font-size: 1.5rem;
            width: 1.5rem;
            &.running {
                color: green;
            }

            &.stopped {
                color: gray;
            }

            &.starting {
                color: yellow;
            }
        }

        .action {

            span {
                font-size: 1.25rem;
                margin: 0 0.25rem;

                &:hover {
                    color: green;
                }
            }
        }
        &:hover {
            background-color: beige;

            .action {
                visibility: visible;
            }
        }
    }
</style>
