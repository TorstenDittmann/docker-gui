<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    import { onMount } from "svelte";

    import Router from "svelte-spa-router";

    import Navigation from "./lib/Navigation.svelte";
    import Container from "./routes/Container.svelte";
    import Containers from "./routes/Containers.svelte";
    import Images from "./routes/Images.svelte";
    import Settings from "./routes/Settings.svelte";
    import { global, state } from "./stores";

    const routes = {
        "/": Containers,
        "/container/:id": Container,
        "/images": Images,
        "/settings": Settings,
    };

    onMount(async () => {
        if ($global.inTauri) {
            invoke("init_process");
            await load();

            //TODO: replace interval
            //setInterval(load, 5000);
        } else {
            state.loadMock();
        }
    });

    const load = async () => {
        await state.containers.load();
        await state.images.load();
    }
</script>

<div class="container">
    <Navigation />
    <main>
        <Router {routes} />
    </main>
</div>

<style lang="scss">
    .container {
        display: grid;
        grid-template-columns: 12em 1fr;
        grid-template-rows: 1fr;
        gap: 0px 0px;
        grid-template-areas: "navigation content";
        height: 100vh;
        max-height: 100vh;
        margin: 0;
        padding: 0;

        main {
            grid-area: content;
            overflow: auto;
            display: flex;
            flex-direction: column;
        }
    }
</style>
