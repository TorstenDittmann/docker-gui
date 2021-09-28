<script lang="ts">
    import { push } from "svelte-spa-router";
    import active from "svelte-spa-router/active";
</script>

<nav>
    <ul>
        <li>
            <h2>Docker GUI</h2>
        </li>
        <li on:click={() => push("/")} use:active={"/"}>Container</li>
        <li on:click={() => push("/images")} use:active={"/images"}>Images</li>
        <li on:click={() => push("/settings")} use:active={"/settings"}>
            Settings
        </li>
    </ul>
</nav>

<style lang="scss">
    nav {
        grid-area: navigation;
        ul {
            transform: none;
            padding: 0;
            list-style: none;
            margin: 0;
            li {
                cursor: pointer;
                padding-left: 1rem;
                font-size: 1.25rem;
                line-height: 2.5rem;
                &:hover {
                    background-color: beige;
                }
                h2 {
                    font-size: 1.5rem;
                    margin: 1rem 0;
                    font-weight: bold;
                }
            }
        }
    }
</style>
