<script lang="ts">
    import {
        Header,
        SideNav,
        SkipToContent,
        Content,
        Grid,
        Row,
        Column,
        HeaderUtilities,
        HeaderGlobalAction,
    } from "carbon-components-svelte";
    import SettingsAdjust20 from "carbon-icons-svelte/lib/SettingsAdjust20";

    import { invoke } from "@tauri-apps/api/tauri";

    import { onMount } from "svelte";

    import Router from "svelte-spa-router";

    import Container from "./routes/Container.svelte";
    import Containers from "./routes/Containers.svelte";
    import Images from "./routes/Images.svelte";
    import Settings from "./routes/Settings.svelte";
    import { global, state } from "./stores";
    import Navigation from "./lib/Navigation.svelte";

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
    };

    let isSideNavOpen = false;
</script>

<Header company="Appwrite" platformName="Docker GUI" bind:isSideNavOpen>
    <div slot="skip-to-content">
        <SkipToContent />
    </div>
    <HeaderUtilities>
        <HeaderGlobalAction aria-label="Settings" icon={SettingsAdjust20} />
    </HeaderUtilities>
</Header>
<SideNav bind:isOpen={isSideNavOpen}>
    <Navigation />
</SideNav>

<Content>
    <Router {routes} />
</Content>
