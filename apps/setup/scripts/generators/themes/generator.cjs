const fs = require("fs");
const path = require("path");

const { artThemes } = require("./data/art.cjs");
const { uiThemes } = require("./data/ui.cjs");

const { FRONTEND_APP_ROOT } = require("../../settings.cjs");

const DEST_PATH = path.resolve(FRONTEND_APP_ROOT, './global/data/themes');

const toPascalCase = (text) => {
  return text.toLowerCase().replace(/(?:^|-)(\w)/g, (_, letter) => letter.toUpperCase());
};

const toCamelCase = (text) => {
  return text.toLowerCase().replace(/-(\w)/g, (_, letter) => letter.toUpperCase());
};

const toSnakeCase = (text) => {
  return text.toLowerCase().replace(/-/g, '_');
};

const toCss = (stl) => {
  const css = {};

  const uiTheme = uiThemes[stl];
  const artTheme = artThemes[stl];

  for (const key of Object.keys(uiThemes[stl].styles)) {
    let items = [];

    const uiStyles = uiThemes[stl].styles[key];
    const artStyles = artThemes[stl].styles[key];

    for (const [name, value] of Object.entries(uiStyles)) {
      items.push(`    --${name}: ${value};`);
    }

    items.push(``);

    for (const [name, value] of Object.entries(artStyles)) {
      items.push(`    --${name}: ${value || '#fff'};`);
    }

    css[key] = items.join('\n');
  }

  return css;
};

const saveStyles = () => {
  const result = [];

  const root = path.resolve(DEST_PATH, './styles');

  fs.mkdirSync(root, { recursive: true });

  for (const [name, { label }] of Object.entries(uiThemes)) {
    const file = `./${name}.ts`;
    const importPath = `./styles/${name}.ts`;
    const dest = path.resolve(root, file);

    const css = toCss(name);

    let data = '';

    data += `export const styles = {\n`;

    for (const [key, value] of Object.entries(css)) {
      data += `  ${key}: \`\n${value}\n  \`,\n`;
    }

    data += `} as const;\n`;

    fs.writeFileSync(dest, data, (err) => {
      if (err) {
        console.error(`Error writing file ${dest}:`, err);
      } else {
        console.log(`File ${dest} written successfully.`);
      }
    });

    result.push({ name, label, file, importPath });``
  }

  return result.sort((a, b) => a.name.localeCompare(b.name));
};

const saveStylesExport = (files) => {
  let data = '';

  for (const { name, importPath } of files) {
    data += `import { styles as ${toCamelCase(name)} } from "${importPath}";\n`;
  }

  data += '\n';

  data += `import { ThemeName } from "./enums";\n`;

  data += '\n';

  data += 'export type ThemeStyle = { light: string, dark: string };\n';

  data += '\n';

  data += `export const styles: Record<ThemeName, ThemeStyle> = {\n`;

  for (const { name } of files) {
    data += `  [ThemeName.${toPascalCase(name)}]: ${toCamelCase(name)},\n`;
  }

  data += `};\n`;

  const dest = path.resolve(DEST_PATH, './styles.ts');

  fs.writeFileSync(dest, data, (err) => {
    if (err) {
      console.error(`Error writing file ${dest}:`, err);
    } else {
      console.log(`File ${dest} written successfully.`);
    }
  });
};

const saveEnumsExport = (files) => {
  let data = '';

  data += `export enum ThemeName {\n`;

  for (const { name } of files) {
    data += `  ${toPascalCase(name)} = "${toSnakeCase(name)}",\n`;
  }

  data += `};\n`;

  data += `\n`;

  data += `export enum ThemeMode {\n`;
  data += `  Light = 'light',\n`;
  data += `  Dark = 'dark',\n`;
  data += `};\n`;

  const dest = path.resolve(DEST_PATH, './enums.ts');

  fs.writeFileSync(dest, data, (err) => {
    if (err) {
      console.error(`Error writing file ${dest}:`, err);
    } else {
      console.log(`File ${dest} written successfully.`);
    }
  });
};

const saveThemesExport = (files) => {
  let data = '';

  data += `import { ThemeStyle, styles } from "./styles";\n`;
  data += `import { ThemeName } from "./enums";\n`;

  data += '\n';

  data += `export type ThemeItem = {\n`;
  data += `  name: ThemeName;\n`;
  data += `  label: string;\n`;
  data += `  styles: ThemeStyle;\n`;
  data += `};\n`;

  data += '\n';

  data += `export const themes: Record<ThemeName, ThemeItem> = {\n`;

  for (const { name, label } of files) {
    data += `  [ThemeName.${toPascalCase(name)}]: {\n`;
    data += `    name: ThemeName.${toPascalCase(name)},\n`;
    data += `    label: "${label}",\n`;
    data += `    styles: styles[ThemeName.${toPascalCase(name)}],\n`;
    data += `  },\n`;
  }

  data += `} as const;\n`;

  const dest = path.resolve(DEST_PATH, './themes.ts');

  fs.writeFileSync(dest, data, (err) => {
    if (err) {
      console.error(`Error writing file ${dest}:`, err);
    } else {
      console.log(`File ${dest} written successfully.`);
    }
  });
};

const saveExport = () => {
  let data = '';

  data += `export * from './themes';\n`;
  data += `export * from './enums';\n`;

  const dest = path.resolve(DEST_PATH, './index.ts');

  fs.writeFileSync(dest, data, (err) => {
    if (err) {
      console.error(`Error writing file ${dest}:`, err);
    } else {
      console.log(`File ${dest} written successfully.`);
    }
  });
};

exports.generate = () => {
  const files = saveStyles();

  saveExport();

  saveEnumsExport(files);
  saveStylesExport(files);
  saveThemesExport(files);
};
