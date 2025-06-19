<script lang="ts">
  let { title, link }: { title: string, link: string } = $props()
</script>

<p class="text-base font-medium text-blue-500 hover:underline cursor-pointer">
  <a href={link}>{title}</a>
</p>