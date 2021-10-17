<script lang="ts">
    import { state } from "../stores";
    import { includes } from "../helpers";
    import {
        DataTable,
        Toolbar,
        ToolbarContent,
        ToolbarSearch,
        ToolbarMenu,
        ToolbarMenuItem,
        TooltipIcon,
        Tag,
        Link,
        ButtonSet,
        Button,
    } from "carbon-components-svelte";
    import PlayFilledAlt16 from "carbon-icons-svelte/lib/PlayFilledAlt16";
    import StopFilledAlt16 from "carbon-icons-svelte/lib/StopFilledAlt16";
    import Restart16 from "carbon-icons-svelte/lib/Restart16";
    import TrashCan16 from "carbon-icons-svelte/lib/TrashCan16";
    import type { TagProps } from "carbon-components-svelte/types/Tag/Tag";
    import { docker } from "../docker";

    let search = "";

    const getState = (state: string): TagProps["type"] => {
        switch (state) {
            case "running":
                return "green";

            case "exited":
                return "red";

            case "kill":
            case "die":
                return "purple";

            default:
                return "gray";
        }
    };

    $: filteredContainers = $state.containers.filter((container) => {
        return search !== ""
            ? container.Names.some((name) => includes(name, search))
            : true;
    });
</script>

<DataTable
    size="short"
    title="Containers"
    description="Your local containers."
    sortable
    headers={[
        { key: "State", value: "State" },
        { key: "Names", value: "Names" },
        { key: "Image", value: "Image" },
        { key: "Actions", empty: true },
    ]}
    rows={filteredContainers}
>
    <Toolbar>
        <ToolbarContent>
            <ToolbarSearch bind:value={search} />
            <ToolbarMenu>
                <ToolbarMenuItem primaryFocus>Restart all</ToolbarMenuItem>
                <ToolbarMenuItem
                    href="https://cloud.ibm.com/docs/loadbalancer-service"
                >
                    API documentation
                </ToolbarMenuItem>
                <ToolbarMenuItem danger>Stop all</ToolbarMenuItem>
            </ToolbarMenu>
        </ToolbarContent>
    </Toolbar>
    <span slot="cell" let:cell let:row>
        {#if cell.key === "State"}
            <Tag type={getState(cell.value)} />
        {:else if cell.key === "Names"}
            <Link href={`/#/container/${row.id}`}>{cell.value}</Link>
        {:else if cell.key === "Actions"}
                <Button
                    icon={PlayFilledAlt16}
                    iconDescription="Start"
                    size="small"
                    kind="ghost"
                    on:click={() => docker.container.start(row.id)}
                />
                <Button
                    icon={StopFilledAlt16}
                    iconDescription="Stop"
                    size="small"
                    kind="ghost"
                    on:click={() => docker.container.stop(row.id)}
                />
                <Button
                    icon={Restart16}
                    iconDescription="Restart"
                    size="small"
                    kind="ghost"
                    on:click={() => docker.container.restart(row.id)}
                />
                <Button
                    icon={TrashCan16}
                    iconDescription="Delete"
                    size="small"
                    kind="ghost"
                    on:click={() => docker.container.delete(row.id)}
                />
        {:else}{cell.value}{/if}
    </span>
</DataTable>

