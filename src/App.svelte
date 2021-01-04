<script lang="ts">
	import { onMount } from "svelte";
	import { execute } from "tauri/api/process";
	import Container from "./Container.svelte";
	
	let containers = [];
	onMount(async () => {
		let ids = (await execute("docker", ["ps", "-q"])).split("\n").filter(Boolean);
		let stdout = await execute("docker", ["inspect", ...ids]);
		containers = JSON.parse(stdout);
		console.log(containers)
	})
</script>

<div class="container">
	<section class="text-gray-400 bg-gray-900 body-font">
		<div class="container px-5 py-5 mx-auto flex flex-wrap">
			<div class="flex flex-wrap w-full">
				<div class="w-full">
					{#each containers as container}
						<Container {container} />
					{/each}
				</div>
			</div>
		</div>
	</section>
</div>

<style global lang="postcss">

  /* only apply purgecss on utilities, per Tailwind docs */
  /* purgecss start ignore */
  @tailwind base;
  @tailwind components;
  /* purgecss end ignore */

  @tailwind utilities;

</style>