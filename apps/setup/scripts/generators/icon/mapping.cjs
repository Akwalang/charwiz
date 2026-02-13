const prefix = {
  png: "app-",
};

const mapping = [
  { type: "png", size: 310, tmp: `${prefix.png}310.png`, dest: "Square310x310Logo.png" },
  { type: "png", size: 284, tmp: `${prefix.png}284.png`, dest: "Square284x284Logo.png" },
  { type: "png", size: 150, tmp: `${prefix.png}150.png`, dest: "Square150x150Logo.png" },
  { type: "png", size: 142, tmp: `${prefix.png}142.png`, dest: "Square142x142Logo.png" },
  { type: "png", size: 107, tmp: `${prefix.png}107.png`, dest: "Square107x107Logo.png" },
  { type: "png", size: 89, tmp: `${prefix.png}89.png`, dest: "Square89x89Logo.png" },
  { type: "png", size: 71, tmp: `${prefix.png}71.png`, dest: "Square71x71Logo.png" },
  { type: "png", size: 44, tmp: `${prefix.png}44.png`, dest: "Square44x44Logo.png" },
  { type: "png", size: 30, tmp: `${prefix.png}30.png`, dest: "Square30x30Logo.png" },
  { type: "png", size: 256, tmp: `${prefix.png}256.png`, dest: "128x128@2x.png" },
  { type: "png", size: 128, tmp: `${prefix.png}128.png`, dest: "128x128.png" },
  { type: "png", size: 32, tmp: `${prefix.png}32.png`, dest: "32x32.png" },
  { type: "png", size: 512, tmp: `${prefix.png}512.png`, dest: "icon.png" },
  { type: "png", size: 50, tmp: `${prefix.png}50.png`, dest: "StoreLogo.png" },

  { type: "icns", size: 512, tmp: "app.icns", dest: "icon.icns" },

  { type: "ico", size: 256, tmp: "app.ico", dest: "icon.ico" },
];

module.exports = { prefix, mapping };
