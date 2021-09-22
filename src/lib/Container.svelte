<script lang="ts">
    import Fa from 'svelte-fa';
    import { faPlay, faStop, faTrash } from '@fortawesome/free-solid-svg-icons';
    import { push } from 'svelte-spa-router';

    import type { Container } from "../stores/state";

    export let container: Container;
</script>

<tr class="container" on:click={() => push(`/container/${container.id}`)}>
    <td>{container.names[0]}</td>
    <td>{container.image}</td>
    <td class="action">
        <span><Fa icon={faPlay} /></span>
        <span><Fa icon={faStop} /></span>
        <span><Fa icon={faTrash} /></span>
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