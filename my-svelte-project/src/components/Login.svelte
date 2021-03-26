<script language="ts">
  import { Base64 } from "js-base64";
  import Input from "./Input.svelte";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  let value = "";
  const secondStage = async (newCredentialsInfo) => {
    try {
      let { rawId, response: responseCred, id, type } = newCredentialsInfo;
      rawId = Base64.fromUint8Array(new Uint8Array(rawId), true);
      console.log(rawId);
      let {
        authenticatorData,
        clientDataJSON,
        signature,
        userHandle,
      } = responseCred;
      authenticatorData = Base64.fromUint8Array(
        new Uint8Array(authenticatorData),
        true
      );
      clientDataJSON = Base64.fromUint8Array(
        new Uint8Array(clientDataJSON),
        true
      );
      signature = Base64.fromUint8Array(new Uint8Array(signature), true);
      userHandle = Base64.fromUint8Array(new Uint8Array(userHandle), true);
      responseCred = {
        authenticatorData,
        clientDataJSON,
        signature,
        userHandle,
      };
      const confirmedResponse = await fetch(`/auth/login/${value}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ id, rawId, response: responseCred, type }),
      });

      const responseConfirmed = await confirmedResponse.json();
      console.log(responseConfirmed);
      dispatch("message", {
        title: "Logged in",
        description: `Congrats ${value}, you have been logged in, welcome to passwordless future !`,
      });
    } catch (e) {
      dispatch("message", {
        title: "Something went wrong",
        description: `Oops I did it again`,
      });
    }
  };
  const handleSubmit = async (event) => {
    const response = await fetch(`/auth/challenge/login/${value}`, {
      method: "POST",
    });

    const publicKey = await response.json();
    let { challenge, allowCredentials } = publicKey.publicKey;
    challenge = Base64.toUint8Array(challenge);
    let newCredentials = allowCredentials.map((item) => {
      let { id, type } = item;
      id = Base64.toUint8Array(id);
      return { type, id };
    });
    const newCredentialsInfo = await navigator.credentials.get({
      publicKey: {
        ...publicKey.publicKey,
        challenge,
        allowCredentials: newCredentials,
      },
    });
    console.log("success", newCredentialsInfo);
    await secondStage(newCredentialsInfo);
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
    <button type="submit">Login</button>
  </form>
</main>
