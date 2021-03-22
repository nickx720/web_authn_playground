<script language="ts">
  export let value;
  export let placeholder;
  export let label;
  export let name;
  export let id;
</script>
<style>
    label{
        margin:0 auto;
        margin-bottom:1rem;
    }
</style>

<label for={id}>{label}</label>
<input bind:value {placeholder} {id} {name} type="text" />
