const { encodeImage } = require("./ravif-js.node");

encodeImage({
  quality: 1,
  speed: 2,
  alpha_quality: 2,
  dirty_alpha: false,
  threads: 3,
});
