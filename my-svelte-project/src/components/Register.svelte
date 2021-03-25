<script language="ts">
  import { Base64 } from "js-base64";
  import Input from "./Input.svelte";
  export let onSubmit;
  export let fields;
  let value = "";
  /* const publicKeyCredentialToJSON = (pubKeyCred) => {
    if (pubKeyCred instanceof Array) {
      let arr = [];
      for (let i of pubKeyCred) arr.push(publicKeyCredentialToJSON(i));

      return arr;
    }

    if (pubKeyCred instanceof ArrayBuffer) {
      return encode(pubKeyCred as any);
    }

    if (pubKeyCred instanceof Object) {
      let obj = {};

      for (let key in pubKeyCred) {
        obj[key] = publicKeyCredentialToJSON(pubKeyCred[key]);
      }

      return obj;
    }

    return pubKeyCred;
  }; */
  const secondStage = async (newCredentialsInfo) => {
    let { rawId, response: responseCred, id, type } = newCredentialsInfo;
    rawId = Base64.fromUint8Array(new Uint8Array(rawId), true);
    console.log(rawId);
    let { attestationObject, clientDataJSON } = responseCred;
    attestationObject = Base64.fromUint8Array(
      new Uint8Array(attestationObject),
      true
    );
    clientDataJSON = Base64.fromUint8Array(
      new Uint8Array(clientDataJSON),
      true
    );
    console.log(attestationObject, clientDataJSON);
    responseCred = { attestationObject, clientDataJSON };
    const confirmedResponse = await fetch(`/auth/register/${value}`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ id, rawId, response: responseCred, type }),
    });

    const responseConfirmed = await confirmedResponse.json();
    console.log(responseConfirmed);
  };
  const handleSubmit = async (event) => {
    const response = await fetch(`/auth/challenge/register/${value}`, {
      method: "POST",
    });

    const publicKey = await response.json();
    console.log(publicKey);
    let { challenge, user } = publicKey.publicKey;
    challenge = Base64.toUint8Array(challenge);
    user.id = Base64.toUint8Array(user.id, true);
    const newCredentialsInfo = await navigator.credentials.create({
      publicKey: { ...publicKey.publicKey, challenge, user },
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
    <button type="submit">Register</button>
  </form>
</main>
