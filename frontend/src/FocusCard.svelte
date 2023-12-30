<script lang="ts">
  import { Button, Card, FAB, TextField } from "m3-svelte";
  import type { Process } from "./types";
  import Icon from "@iconify/svelte";
  import { deleteProcess, sendkillsignal, patchprocess } from "./api";
  import Terminal from "./Terminal.svelte";
  import { createEventDispatcher } from "svelte";

  export let process: Process;
  export let select: (id: number) => void;

  let termtext;
  let io: any;
  let term: any;

  let { name, command, user, dir } = process;

  let d = createEventDispatcher();

  async function save() {
    let resp = await patchprocess(process.id, name, command, user, dir);
    let id = await resp.json();
    select(id);
  }

  let socket = new WebSocket(
    `${location.protocol === "http:" ? "ws:" : "wss:"}${location.hostname}${
      location.port ? `:${location.port}` : ""
    }/api/${process.id}/tail`
  );
  socket.addEventListener("message", async (event) => {
    let text = await event.data.text();
    io.print(text.replaceAll("\n", "\r\n"));
  });

  fetch(`/api/${process.id}`).then(async (r) => {
    termtext = await r.text();
    io.print(termtext.replaceAll("\n", "\r\n"));
    term.setCursorVisible(0);
  });
</script>

<div class="m-4">
  <Card type="elevated">
    <div class="flex flex-col gap-y-5">
      <h1 class="text-3xl">{name}</h1>

      <div class="flex">
        <div class="flex-1">
          <Terminal bind:io bind:term />
        </div>
        <div class="flex justify-between">
          <div class="flex flex-col gap-y-5 p-3 items-end">
            <TextField name="Name" bind:value={name} />
            <TextField name="command" bind:value={command} />
            <TextField name="In directory" bind:value={dir} />
            <TextField name="as user" bind:value={user} />

            <div>
              <FAB on:click={save} icon="clarity:floppy-solid" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </Card>
</div>
