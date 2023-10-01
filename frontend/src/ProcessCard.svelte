<script lang="ts">
  import { Button, Card } from "m3-svelte";
  import type { Process } from "./types";
  import Icon from "@iconify/svelte";
  import { deleteProcess, sendkillsignal, startprocess } from "./api";

  export let process: Process;
  export let selectedprocess: Process | null;
</script>

<button on:click={() => (selectedprocess = process)}>
  <Card type={process === selectedprocess ? "outlined" : "elevated"}>
    <div class="flex flex-col items-start gap-y-5">
      <div class="text-xl">
        "{process.command}"
        {process.exited == null
          ? "running"
          : `exited with code ${process.exited}`}
      </div>

      <div class="w-full flex justify-end gap-x-3">
        {#if process.exited != null}
          <Button
            type="elevated"
            iconType="full"
            on:click={() => {
              deleteProcess(process.id);
            }}
          >
            <Icon icon="clarity:trash-solid" />
          </Button>
          <Button
            on:click={() => {
              startprocess(process.id);
            }}
            iconType="full"
            type="elevated"
          >
            <Icon icon="codicon:debug-start" />
          </Button>
        {:else}
          <Button
            on:click={() => {
              sendkillsignal(process.id, 2); // SIGINT
            }}
            iconType="full"
            type="elevated"
          >
            <Icon icon="clarity:stop-solid" />
          </Button>
        {/if}
        <Button
          iconType="full"
          on:click={() => (selectedprocess = process)}
          type="tonal"
        >
          <Icon icon="clarity:pencil-solid" />
        </Button>
      </div>
    </div>
  </Card>
</button>
