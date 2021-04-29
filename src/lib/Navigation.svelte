<script lang="ts">
import active from 'svelte-spa-router/active'
import { invoke } from '@tauri-apps/api/tauri'

function test() {

// then call it:
invoke('plugin:docker|my_custom_command').then(response => {
  console.log(response)
}).catch(error => {
  // do something with the Err() response string
  console.error(error)
})
	}

</script>
<nav>
    <ul class="menu bg-gray">
        <li class="menu-item">
            <div class="tile tile-centered">
                <div class="tile-icon">
                    <figure class="avatar" data-initial="D" />
                </div>
                <div class="tile-content">Docker GUI</div>
            </div>
        </li>
        <li class="divider" />
        <li class="menu-item">
            <a  use:active={'/'} href="#/">Container</a>
        </li>
        <li class="menu-item">
            <a href="#/images" use:active={'/images'}>Images</a>
        </li>
        <li class="menu-item">
            <a href="#/settings" use:active={'/settings'}>Settings</a>
        </li>
        <li class="menu-item">
          <a on:click|once={test}>Test</a>
      </li>
    </ul>
</nav>

<style lang="scss">
    nav {
        grid-area: navigation;

        .menu {
            height: 100%;
            transform: none;
        }
    }
</style>
