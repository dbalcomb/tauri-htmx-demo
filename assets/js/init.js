xhook.before(function (request, callback) {
  if (request.url.match("/contact/")) {
    window.__TAURI__
      .invoke("htmx", request)
      .then((contents) => {
        callback({
          status: 200,
          statusText: "OK",
          text: contents,
          data: contents,
          headers: {
            "content-length": contents.length,
            "content-type": "text/html",
          },
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
