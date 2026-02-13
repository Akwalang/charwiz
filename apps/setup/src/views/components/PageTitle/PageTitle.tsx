interface PageTitleProps {
  title: string,
  description: string,
}

export const PageTitle: React.FC<PageTitleProps> = ({ title, description }) => {
  return (
    <>
      <h1 className="text-2xl font-semibold mb-2">{title}</h1>
      <p className="text-foreground/60 mb-8">{description}</p>
    </>
  );
};
