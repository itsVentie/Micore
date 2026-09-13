import { useState } from 'preact/hooks';
import { Sidebar, type ModuleRoute } from '../components/Sidebar';
import { ContactsModule } from '../modules/contacts/ContactsModule';

interface DashboardPageProps {
  onLock: () => void;
}

export function DashboardPage({ onLock }: DashboardPageProps) {
  const [currentRoute, setCurrentRoute] = useState<ModuleRoute>('contacts');

  return (
    <div className="flex flex-1 overflow-hidden">
      <Sidebar
        currentRoute={currentRoute}
        onRouteChange={(route) => setCurrentRoute(route)}
        onLock={onLock}
      />

      <section className="flex-1 overflow-hidden">
        {currentRoute === 'contacts' && <ContactsModule />}
        {currentRoute === 'bookmarks' && (
          <div className="p-6 text-xs text-zinc-500">Bookmark Manager Module (Pending)</div>
        )}
        {currentRoute === 'snippets' && (
          <div className="p-6 text-xs text-zinc-500">Code Snippets & Vault Module (Pending)</div>
        )}
        {currentRoute === 'knowledge' && (
          <div className="p-6 text-xs text-zinc-500">Knowledge Base Module (Pending)</div>
        )}
      </section>
    </div>
  );
}