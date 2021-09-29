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

<tr class="container">

    <td on:click={() => push(`/container/${container.id}`)}>
        <span
            class="state"
            class:running={container.state === "running"}
            class:stopped={container.state === "exited"}
            class:starting={['kill', 'die'].includes(container.state)}
        >
            <Fa icon={faServer} />
        </span>
        {container.names[0]}
    </td>
    <td>{container.image}</td>
    <td class="action">
        <span on:click={() => docker.container.start(container.id)}>
            <Fa icon={faPlay} />
        </span>
        <span on:click={() => docker.container.stop(container.id)}>
            <Fa icon={faStop} />
        </span>
        <span on:click={() => docker.container.restart(container.id)}>
            <Fa icon={faSync} />
        </span>
        <span on:click={() => docker.container.delete(container.id)}>
            <Fa icon={faTrash} />
        </span>
    </td>
</tr>

<style lang="scss">
    .container {
        line-height: 3rem;
        cursor: pointer;

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
