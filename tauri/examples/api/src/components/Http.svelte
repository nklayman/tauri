<script>
  import { request } from "@tauri-apps/api/http";
  let httpMethod = "GET";
  let httpUrl = "";
  let httpBody = "";

  export let onMessage;

  function makeHttpRequest() {
    let method = httpMethod || "GET";
    let url = httpUrl || "";
    let body = httpBody || "";

    const options = {
      url: url || "",
      method: method || "GET"
    };

    if (
      (body.startsWith("{") && body.endsWith("}")) ||
      (body.startsWith("[") && body.endsWith("]"))
    ) {
      body = JSON.parse(body);
    } else if (body.startsWith("/") || body.match(/\S:\//g)) {
      options.bodyAsFile = true;
    }
    options.body = body;

    request(options)
      .then(onMessage)
      .catch(onMessage);
  }
</script>

<form on:submit|preventDefault={makeHttpRequest}>
  <select class="button" id="request-method" bind:value={httpMethod}>
    <option value="GET">GET</option>
    <option value="POST">POST</option>
    <option value="PUT">PUT</option>
    <option value="PATCH">PATCH</option>
    <option value="DELETE">DELETE</option>
  </select>
  <input id="request-url" placeholder="Type the request URL..." bind:value={httpUrl} />
  <br />
  <textarea id="request-body" placeholder="Request body" rows="5" style="width:100%;margin-right:10px;font-size:12px"
    bind:value={httpBody} />
  <button class="button" id="make-request">
    Make request
  </button>
</form>