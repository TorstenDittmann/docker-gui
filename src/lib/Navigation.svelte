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
    <SideNavMenu text="Menu">
        <SideNavMenuItem href="/" text="Link 1" />
        <SideNavMenuItem href="/" text="Link 2" />
        <SideNavMenuItem href="/" text="Link 3" />
    </SideNavMenu>
    <SideNavDivider />
    <SideNavLink text="Link 4" href="/" />
</SideNavItems>
