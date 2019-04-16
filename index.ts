import("./pkg/pcm_visual").then(module => {
  // won't typecheck if yourlib does not expose the run function
  module.run();
});