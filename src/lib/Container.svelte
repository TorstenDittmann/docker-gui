<script lang="ts">
    import Fa from 'svelte-fa';
    import { faPlay, faStop, faSync, faTrash } from '@fortawesome/free-solid-svg-icons';
    import { push } from 'svelte-spa-router';

    import type { Container } from "../stores/state";
import { docker } from '../docker';

    export let container: Container;
</script>

<tr class="container">
    <td on:click={() => push(`/container/${container.id}`)}>{container.names[0]}</td>
    <td on:click={() => push(`/container/${container.id}`)}>{container.image}</td>
    <td class="action">
        <span on:click={() => docker.container.start(container.id)}><Fa icon={faPlay} /></span>
        <span on:click={() => docker.container.stop(container.id)}><Fa icon={faStop} /></span>
        <span on:click={() => docker.container.restart(container.id)}><Fa icon={faSync} /></span>
        <span on:click={() => docker.container.delete(container.id)}><Fa icon={faTrash} /></span>
    </td>
</tr>

<style lang="scss">
    .container {
        line-height: 3rem;
        cursor: pointer;

        .action {
            visibility: hidden;

            span {
                font-size: 1.25rem;
                margin: 0 .25rem;

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