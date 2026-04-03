import { MenuBar } from './MenuBar';
import { StatusBar } from './StatusBar';
import { PanelLayout } from './PanelLayout';

export function AppLayout() {
  return (
    <div className="flex flex-col h-screen bg-white dark:bg-gray-900">
      <MenuBar />
      <PanelLayout />
      <StatusBar />
    </div>
  );
}
