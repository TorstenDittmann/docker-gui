<script lang="ts">
  import Router from "svelte-spa-router";

  import Navigation from "./lib/Navigation.svelte";
  import Containers from "./routes/Containers.svelte";
  import Images from "./routes/Images.svelte";
  import Settings from "./routes/Settings.svelte";

  const routes = {
    // Exact path
    "/": Containers,

    // Using named parameters, with last being optional
    "/images": Images,

    // Wildcard parameter
    "/settings": Settings,

    // Catch-all
    // This is optional, but if present it must be the last
    "*": Containers,
  };
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
