import { useEffect, useState } from "react";

import { Loader, Cache } from "@/global/classes";

interface SvgImageProps {
  src: string;
  style?: React.CSSProperties;
}

const LOADER = new Loader<string>(
  new Cache(10),
  async (data) => {
    const svg = await data.text();
    return svg.replace(/^.*(<svg\s)/is, "$1");
  },
);

export const SvgImage: React.FC<SvgImageProps> = (props) => {
  const { src, ...other } = props;

  const [svg, setSvg] = useState<string | null>(LOADER.get(src));

  useEffect(() => {
    svg || LOADER.load(src).then(setSvg).catch((e) => console.error(e.message));
  }, [props.src]);

  return (
    <div {...other} dangerouslySetInnerHTML={{ __html: svg || "" }} />
  );
};
