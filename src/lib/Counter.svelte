<script lang="ts">
  import { invoke } from "@tauri-apps/api";

  let count: number = 0;
  const increment = () => {
    count += 1;

    invoke("greet", { name: "World" })
      // `invoke` returns a Promise
      .then((response) => console.log(response));
  };
</script>

<button on:click={increment}>
  count is {count}
</button>
