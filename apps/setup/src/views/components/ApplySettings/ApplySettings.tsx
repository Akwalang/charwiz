interface ApplySettingsProps {}

// <button className="px-6 py-2 rounded-lg bg-secondary text-secondary-foreground hover:bg-secondary/60 transition-colors">
//   Отмена
// </button>

export const ApplySettings: React.FC<ApplySettingsProps> = () => {
  return (
    <div className="sticky bottom-0 mt-7 py-3 flex justify-end bg-background">
      <div className="flex gap-4">
        <button className="px-6 py-2 rounded-lg bg-primary text-primary-foreground hover:bg-primary/80 transition-colors">
          Save changes
        </button>
      </div>
    </div>
  );
};
