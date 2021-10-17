<script lang="ts">
    import {
        SideNavDivider,
        SideNavItems,
        SideNavLink,
        SideNavMenuItem,
        SideNavMenu,
    } from "carbon-components-svelte";

    import { location } from "svelte-spa-router";
</script>

<SideNavItems>
    <SideNavLink text="Containers" href="/#/" isSelected={$location === '/'} />
    <SideNavLink text="Images" href="/#/images" isSelected={$location === '/images'} />
    <SideNavLink text="Settings" href="/#/settings" isSelected={$location === '/settings'} />
    <SideNavDivider />
    <SideNavLink text="Link 4" href="/" />
</SideNavItems>
