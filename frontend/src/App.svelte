<script lang="ts">
  import {
    Button,
    Card,
    Dialog,
    FAB,
    SnackbarAnim,
    StyleFromScheme,
    TextField,
  } from "m3-svelte";

  import type { SnackbarIn } from "m3-svelte/package/containers/Snackbar.svelte";

  import Terminal from "./Terminal.svelte";
  import { onMount } from "svelte";
  import Icon from "@iconify/svelte";
  import ProcessCard from "./ProcessCard.svelte";
  import type { Process } from "./types";
  import FocusCard from "./FocusCard.svelte";
  import New from "./New.svelte";
  import { setsnackbar } from "./api";

  let shownew = false;

  let snackbar: (data: SnackbarIn) => void;

  let command = "echo 1";
  let user = "root";

  export async function refresh() {
    let r = await fetch("/api/list");
    data = await r.json();
    if (!Object.values(data).find((p) => p.id == selectedprocess?.id)) {
      selectedprocess = null;
    }
  }

  let data: any;
  let selectedprocess: Process;

  async function selectprocess(id) {
    await refresh();
    selectedprocess = Object.values(data).find((p) => p.id == id);
  }

  onMount(async () => {
    refresh();
    setsnackbar(snackbar);
  });
</script>

<main>
  {#key selectedprocess}
    {#if selectedprocess}
      <FocusCard
        on:refresh={refresh}
        process={selectedprocess}
        select={selectprocess}
      />
    {/if}
  {/key}
  <!-- <hr/> -->

  {#if data}
    <div class="grid m-4">
      {#each Object.values(data) as process}
        <ProcessCard
          on:refresh={refresh}
          select={selectprocess}
          {process}
          bind:selectedprocess
        />
      {/each}
    </div>
  {/if}

  <!-- <Dialog display="a" headline="Select Signal to send" bind:open={showsignals}>
    <TextField name="signal" bind:value={killsignal} />
    <Button type="tonal" on:click={sendkillsignal}>Kill</Button>
  </Dialog> -->

  <New on:refresh={refresh} bind:show={shownew} />

  <div id="fab">
    <FAB icon="charm:plus" on:click={() => (shownew = true)} />
  </div>
  <svelte:component this={SnackbarAnim} bind:show={snackbar} />
</main>

<StyleFromScheme
  lightScheme={{
    primary: 4282411062,
    onPrimary: 4294967295,
    primaryContainer: 4290834352,
    onPrimaryContainer: 4278198784,
    inversePrimary: 4289057685,
    secondary: 4283720525,
    onSecondary: 4294967295,
    secondaryContainer: 4292339917,
    onSecondaryContainer: 4279377678,
    tertiary: 4281886056,
    onTertiary: 4294967295,
    tertiaryContainer: 4290571246,
    onTertiaryContainer: 4278198306,
    error: 4290386458,
    onError: 4294967295,
    errorContainer: 4294957782,
    onErrorContainer: 4282449922,
    background: 4294507505,
    onBackground: 4279835927,
    surface: 4294507505,
    onSurface: 4279835927,
    surfaceVariant: 4292863191,
    onSurfaceVariant: 4282599487,
    inverseSurface: 4281217579,
    inverseOnSurface: 4293915368,
    outline: 4285757806,
    outlineVariant: 4291020988,
    shadow: 4278190080,
    scrim: 4278190080,
    surfaceDim: 4292402130,
    surfaceBright: 4294507505,
    surfaceContainerLowest: 4294967295,
    surfaceContainerLow: 4294112747,
    surfaceContainer: 4293717989,
    surfaceContainerHigh: 4293323232,
    surfaceContainerHighest: 4292994266,
    surfaceTint: 4282411062,
  }}
  darkScheme={{
    primary: 4294945779,
    onPrimary: 4284153947,
    primaryContainer: 4291690702,
    onPrimaryContainer: 4294967295,
    inversePrimary: 4289265833,
    secondary: 4294945779,
    onSecondary: 4284153947,
    secondaryContainer: 4286709890,
    onSecondaryContainer: 4294956533,
    tertiary: 4294947764,
    onTertiary: 4285005846,
    tertiaryContainer: 4292294218,
    onTertiaryContainer: 4294967295,
    error: 4294948011,
    onError: 4285071365,
    errorContainer: 4287823882,
    onErrorContainer: 4294957782,
    background: 4279702038,
    onBackground: 4293713893,
    surface: 4279702038,
    onSurface: 4293713893,
    surfaceVariant: 4283319371,
    onSurfaceVariant: 4291936971,
    inverseSurface: 4293713893,
    inverseOnSurface: 4281675315,
    outline: 4288318869,
    outlineVariant: 4283319371,
    shadow: 4278190080,
    scrim: 4278190080,
    surfaceDim: 4279702038,
    surfaceBright: 4282267452,
    surfaceContainerLowest: 4279373073,
    surfaceContainerLow: 4280293918,
    surfaceContainer: 4280557090,
    surfaceContainerHigh: 4281280557,
    surfaceContainerHighest: 4282004280,
    surfaceTint: 4294030310,
  }}
/>

<style>
  .grid {
    display: grid;
    gap: 1rem;
    grid-auto-flow: dense;
    justify-content: space-between;
    grid-template-columns: repeat(auto-fill, minmax(40rem, 1fr));
  }
  #fab {
    position: absolute;
    right: 1em;
    bottom: 1em;
  }
</style>
