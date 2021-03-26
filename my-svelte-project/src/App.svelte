<script lang="ts">
  import Login from "./components/Login.svelte";
  import Register from "./components/Register.svelte";
  import { toasts, ToastContainer, FlatToast } from "svelte-toasts";
  export let name: string;
  name = "Web authentication";

  const showToast = (event) => {
    const { title, description } = event.detail;
    const toast = toasts.add({
      title,
      description,
      duration: 10000, // 0 or negative to avoid auto-remove
      placement: "top-right",
      type: "info",
      theme: "dark",
    });

    // toast.remove()
  };
</script>

<main>
  <h1>Hello {name}!</h1>
  <Register on:message={showToast} />
  <Login  on:message={showToast}/>
  <ToastContainer let:data>
    <FlatToast {data} />
    <!-- Provider template for your toasts -->
  </ToastContainer>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
    word-break: break-all;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
