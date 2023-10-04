xhook.before(function (request, callback) {
  if (request.url.startsWith("/")) {
    window.__TAURI__
      .invoke("htmx", { request })
      .then((response) => {
        callback({
          status: response.status,
          text: response.body,
          data: response.body,
          headers: response.headers,
        });
      })
      .catch((error) => {
        console.error(error);
      });
  } else {
    callback(false);
  }
});

document.addEventListener("htmx:afterSettle", function () {
  window.__TAURI__.invoke("window_did_finish_loading");
}, { once: true });
