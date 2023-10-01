<script lang="ts">
  import { onMount } from "svelte";
  let div: HTMLDivElement;

  export let term;
  export let io;
  const t = new hterm.Terminal();

  onMount(() => {
    t.decorate(div);
    t.onTerminalReady = async () => {
      io = t.io.push();
      term = t;
      let frame = div.querySelector("iframe")!;
      frame.style = "height:100%; width:100%";
      div.style.borderWidth = "1px";

      let screen = frame.contentDocument!.querySelector("x-screen")!;

      screen.style.overflow = "hidden";
      t.installKeyboard();
      io.onVTKeystroke = () => {};
      io.sendString = () => {};
      t.setCursorVisible(0);
    };
  });
</script>

<div bind:this={div} class="term" />

<style>
  div {
    height: 100%;
    /* border: solid px blue; */
    border-radius: 10px;
    overflow: hidden;
  }
</style>
