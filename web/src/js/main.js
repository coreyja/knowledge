import {
  Application,
  Controller,
} from "https://unpkg.com/@hotwired/stimulus/dist/stimulus.js";
window.Stimulus = Application.start();

Stimulus.register(
  "loader",
  class extends Controller {
    static targets = [];

    connect() {
      console.log("loader connected");
      setTimeout(() => {
        console.log("Reloading");
        window.location.reload();
      }, 500);
    }
  }
);
