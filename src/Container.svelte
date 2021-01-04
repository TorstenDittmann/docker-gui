<script lang="ts">
  export let container: any;
</script>

<div class="flex relative pb-12">
  <div class="flex-shrink-0 w-10 h-10 rounded-full bg-green-500 inline-flex items-center justify-center text-white relative z-10">
    <svg fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" class="w-5 h-5" viewBox="0 0 24 24">
      <path d="M22 12h-4l-3 9L9 3l-3 9H2"></path>
    </svg>
  </div>
  <div class="flex-grow pl-4">
    <h2 class="font-medium title-font text-sm text-white mb-1 tracking-wider">{container.Name} - <i class="text-gray-700	">{container.Config.Image}</i></h2>
    <p class="leading-relaxed">{container.State.Status}</p>
  </div>
</div>