interface LogoProps {}

export const Logo: React.FC<LogoProps> = () => {
  return (
    <div className="select-none">
      <h2 className="text-lg font-semibold">Charwiz</h2>
      <p className="text-sm text-foreground/60">Configurations</p>
    </div>
  );
};
