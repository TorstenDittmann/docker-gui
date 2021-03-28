<script lang="ts">
import active from 'svelte-spa-router/active'

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
