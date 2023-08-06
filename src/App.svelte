<script lang="ts">
  import {invoke} from "@tauri-apps/api/tauri";
  import {listen} from "@tauri-apps/api/event";

  $: portName = null;

  const connect = () => {
    invoke("connect").then((response) => {
      portName = response;
    });
  };

  const listenSerial = () => {
    invoke("listen").then((a) => {
      console.log(a);
    });
  };

  listen("event-name", (event) => {
    console.log("Front-end event");
    console.log(event);
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
  });
</script>

<main class="">
  {#if portName}
    <span>{portName}</span>
  {:else}
    <span>No port found</span>
  {/if}
  <button on:click={connect} class="p-2 bg-white/10">Run</button>
  <button on:click={listenSerial} class="p-2 bg-white/10">Listen</button>
</main>
