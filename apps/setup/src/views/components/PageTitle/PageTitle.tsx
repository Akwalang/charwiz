interface PageTitleProps {
  title: string,
  description: string,
}

export const PageTitle: React.FC<PageTitleProps> = ({ title, description }) => {
  return (
    <div className="flex flex-col gap-1 mb-8">
      <h1 className="text-2xl font-semibold">{title}</h1>
      <p className="text-foreground/60">{description}</p>
    </div>
  );
};
