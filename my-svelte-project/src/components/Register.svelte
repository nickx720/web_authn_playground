<script language="ts">
  import Input from "./Input.svelte";
  export let onSubmit;
  export let fields;
  let value = "";
  const handleSubmit = (event) => {
    const data = new FormData(event.target);
    for (const [name, value] of data) {
      console.log(name, value);
    }
    fetch("https://jsonplaceholder.typicode.com/posts", {
      method: "POST",
      body: JSON.stringify({
        title: "foo",
        body: "bar",
        userId: 1,
      }),
      headers: {
        "Content-type": "application/json; charset=UTF-8",
      },
    })
      .then((response) => response.json())
      .then((json) => (value = json.title));
  };
</script>

<main>
  <form on:submit|preventDefault={(event) => handleSubmit(event)}>
    <Input
      bind:value
      name="name"
      label="Name"
      id="name"
      placeholder="Enter name..."
    />
    <button type="submit">Register</button>
  </form>
</main>
