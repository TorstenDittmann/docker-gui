<header>
    <slot />
</header>

<style lang="scss">

    header {
        position: sticky;
        top: 0;
        min-height: 4rem;
        width: 100%;
        display: flex;
        align-items: center;
        background-color: white;

        :global(*) {
            font-size: 1.25rem;
        }
    }
</style>