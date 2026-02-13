const fs = require("fs");
const path = require("path");
const gen = require("icon-gen");

const { prefix, mapping } = require("./mapping.cjs");

const { WEB_PUBLIC_ROOT, BACKEND_APP_ROOT } = require("../../settings.cjs");

const ICON_FOLDER = path.resolve(BACKEND_APP_ROOT, "./icons");

const options = {
  ico: {
    name: "",
    sizes: mapping.filter((item) => item.type === "ico").map((item) => item.size),
  },
  icns: {
    name: "",
    sizes: mapping.filter((item) => item.type === "icns").map((item) => item.size),
  },
  favicon: {
    name: prefix.png,
    pngSizes: mapping.filter((item) => item.type === "png").map((item) => item.size),
  },
};

const generateIcons = async () => {
  const src = path.resolve(__dirname, "./data/icon.svg");
  const dest = path.resolve(__dirname, "./temp");

  if (!fs.existsSync(dest)) fs.mkdirSync(dest);

  try {
    await gen(src, dest, options);
  } catch (error) {
    console.error("Error generating icons:", error);
    throw error;
  }
};

const copyFavicon = () => {
  const src = path.resolve(__dirname, "./data/icon.svg");
  const dest = path.resolve(WEB_PUBLIC_ROOT, "favicon.svg");

  if (fs.existsSync(src)) {
    fs.copyFileSync(src, dest);
    console.log(`Copied ${src} to ${dest}`);
  } else {
    console.error(`Source file ${src} does not exist`);
  }
};

const copyIcons = () => {
  mapping.forEach((item) => {
    const src = path.resolve(__dirname, "temp", item.tmp);
    const dest = path.resolve(ICON_FOLDER, item.dest);

    if (fs.existsSync(src)) {
      fs.copyFileSync(src, dest);
      console.log(`Copied ${src} to ${dest}`);
    } else {
      console.error(`Source file ${src} does not exist`);
    }
  });
};

exports.generate = async () => {
  await generateIcons();

  copyFavicon();
  copyIcons();
};
