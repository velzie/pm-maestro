<script lang="ts">
  import { Button, Card, FAB, TextField, Switch } from "m3-svelte";
  export let checked: boolean;
</script>

<div class="outer">
  <Card type="filled">
    <div class="inner">
      <div class="text">
        <slot />
      </div>
      <label for={undefined}>
        <Switch bind:checked />
      </label>
    </div>
  </Card>
</div>

<style>
  .outer {
    width: 100%;
  }
  .text {
    font-size: 1.1em;
    padding-right: 1em;
  }
  .inner,
  label {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
</style>
