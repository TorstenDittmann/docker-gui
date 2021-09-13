<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import { containers } from './stores/containers';

  import Router from "svelte-spa-router";

  import Navigation from "./lib/Navigation.svelte";
  import Containers from "./routes/Containers.svelte";
  import Images from "./routes/Images.svelte";
  import Settings from "./routes/Settings.svelte";

  const routes = {
    "/": Containers,
    "/images": Images,
    "/settings": Settings,
  };

  onMount(async () => {
    try {
      const response = await invoke('containers_list')
      containers.set(JSON.parse(<string> response));
      console.log($containers);
    } catch (error) {
      alert(error)
      console.error(error)
    }

  });
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
    grid-template-columns: 16em 1fr;
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
      align-items: center;
      flex-direction: column;
    }
  }
</style>
