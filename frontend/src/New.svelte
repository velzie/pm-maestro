<script lang="ts">
  import { Button, Dialog, TextField } from "m3-svelte";
  import { createEventDispatcher } from "svelte";

  export let show = false;

  let name = "New Process";

  let command = "";
  let user = "root";
  let dir = "/";

  let d = createEventDispatcher();
  async function create() {
    let a = await fetch("/api/new", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        command,
        user,
        name,
        dir,
      }),
    });
    d("refresh");
  }
</script>

<Dialog display="a" headline="Spawn New Process" bind:open={show}>
  <div class="flex flex-col space-y-3">
    <TextField name="name" bind:value={name} />
    <TextField name="command" bind:value={command} />
    <TextField name="as user" bind:value={user} />
    <TextField name="working directory" bind:value={dir} />

    <Button type="tonal" on:click={create}>Create</Button>
  </div>
</Dialog>
