import { MenuBar } from './MenuBar';
import { StatusBar } from './StatusBar';
import { PanelLayout } from './PanelLayout';

export function AppLayout() {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh', overflow: 'hidden' }}>
      <MenuBar />
      <PanelLayout />
      <StatusBar />
    </div>
  );
}
