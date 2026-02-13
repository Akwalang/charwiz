import { IconType } from '@/global/data/icons';

import { mapping } from './mapping';

interface IconProps extends React.HTMLAttributes<SVGElement> {
  icon: IconType;
  size: number;
}

export { IconType };

export const Icon: React.FC<IconProps> = (props) => {
  const { icon, ...other } = props;

  const IconComponent = mapping[icon];

  return <IconComponent {...other} />;
};
