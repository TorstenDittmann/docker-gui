<script lang="ts">
  import Logo from "./assets/svelte.png";
</script>

<div class="container">
  <nav>
    <ul class="menu">
      <li class="menu-item">
        <div class="tile tile-centered">
          <div class="tile-icon"><img class="avatar" src={Logo} alt="Avatar"></div>
          <div class="tile-content">Docker GUI</div>
        </div>
      </li>
      <li class="divider"></li>
      <li class="menu-item">
        <a class="active" href="#menus">Container</a>
      </li>
      <li class="menu-item">
        <a href="#menus">Images</a>
      </li>
      <li class="menu-item">
        <a href="#menus">Settings</a>
      </li>
    </ul>
  </nav>
  <main>
    Lorem ipsum dolor, sit amet consectetur adipisicing elit. Sint atque, eaque inventore necessitatibus doloremque expedita harum facere sapiente autem ab velit impedit repellat quasi laborum est eveniet pariatur accusantium esse.
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

    nav {
      grid-area: navigation;

      .menu {
        height: 100%;
      }
    }

    main {
      grid-area: content;
      overflow: auto;
    }
  }
</style>
