<script>
 export let stream;
</script>

{#await $stream then result}
 {result || ""}
{:catch error}
 <span style="color: red">{error.message}</span>
{/await}
