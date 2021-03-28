<div class="container tile tile-centered p-2 c-hand">
    <div class="tile-icon">
        <figure class="avatar" data-initial="MN">
            <i class="avatar-presence online" />
        </figure>
    </div>
    <div class="tile-content">
        <div class="tile-title">
            Machine Name <code>Image Name</code>
        </div>
        <small class="tile-subtitle text-gray"
            >Port: 80</small
        >
    </div>
    <div class="tile-action mr-2">
        <div class="btn-group btn-group-block">
            <button class="btn tooltip tooltip-left" data-tooltip="CLI">
                <i class="icon icon-arrow-right" />
            </button>
            <button
                class="btn tooltip tooltip-left"
                data-tooltip="Open in Browser"
            >
                <i class="icon icon-share" />
            </button>
            <button class="btn tooltip tooltip-left" data-tooltip="Restart">
                <i class="icon icon-refresh" />
            </button>
            <button class="btn tooltip tooltip-left" data-tooltip="Stop">
                <i class="icon icon-stop" />
            </button>
            <button class="btn tooltip tooltip-left" data-tooltip="Delete">
                <i class="icon icon-delete" />
            </button>
        </div>
    </div>
</div>

<style lang="scss">
    @import '../variables';

    .container {
        &:hover {
            background-color: $secondary-color;
        }
    }
</style>