<script lang="ts">
    export let id: string;
    export let names: string[];
    export let image: string;
    export let state: string;
    export let status: string;
</script>

<tr class="container">
    <td>{names[0]}</td>
    <td>{image}</td>
</tr>

<style lang="scss">
    .container {
        line-height: 2rem;
        cursor: pointer;
        &:hover {
            background-color: beige;
        }
    }
</style>