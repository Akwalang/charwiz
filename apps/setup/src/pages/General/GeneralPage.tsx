import { ApplySettings } from "@/views/components";

interface GeneralPageProps {}

export const GeneralPage: React.FC<GeneralPageProps> = () => {
  return (
    <>
      <h1 className="text-2xl font-semibold mb-2">General settings</h1>
      <p className="text-gray-500 mb-8">Manage global keyboard shortcuts</p>

      <section>
        <h2 className="text-sm font-semibold text-gray-400 uppercase mb-4">System</h2>

        <div className="flex items-center justify-between border rounded-xl px-6 py-4">
          <p className="font-medium">Enable Charwiz</p>

          <label className="inline-flex items-center cursor-pointer relative">
            <input type="checkbox" className="sr-only peer" checked />
            <div className="w-11 h-6 bg-gray-200 rounded-full peer-checked:bg-indigo-600 transition-colors"></div>
            <div className="absolute left-1 top-1 w-4 h-4 bg-white rounded-full transition-transform peer-checked:translate-x-5"></div>
          </label>
        </div>
      </section>

      <ApplySettings />
    </>
  );
};
