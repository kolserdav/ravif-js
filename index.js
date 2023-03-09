// @ts-check

const path = require('path');
// @ts-ignore
const { encodeImage, sayHello, scaleImage } = require("./ravif-js.node");

sayHello({ test: true });

encodeImage({
  quality: 1,
  speed: 2,
  alphaQuality: 2,
  dirtyAlpha: false,
  threads: 3,
  filePath: path.resolve(__dirname, 'tmp/1.jpeg'),
  destPath: path.resolve(__dirname, 'tmp/2.avif')
});
/*
scaleImage({
  filePath: path.resolve(__dirname, 'tmp/22.avif'),
  destPath: path.resolve(__dirname, 'tmp/33.avif'),
  width: 100,
  height: 100
});
*/