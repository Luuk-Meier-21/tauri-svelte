<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  const func = () => {
    console.log("hi");
    invoke("greet");
  };

  listen("event-name", (event) => {
    console.log("Front-end event");
    console.log(event);
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
  });
</script>

<main class="">
  <button on:click={func} class="p-2 bg-white/10"> Run </button>
</main>
