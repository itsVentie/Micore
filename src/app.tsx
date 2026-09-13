import { useState, useEffect } from 'preact/hooks';
import { unlockVault, lockVault, checkVaultStatus, touchActivity } from './api/vault';
import { UnlockPage } from './pages/UnlockPage';
import { DashboardPage } from './pages/DashboardPage';

export function App() {
  const [isUnlocked, setIsUnlocked] = useState<boolean>(false);

  useEffect(() => {
    const handleUserActivity = () => {
      if (isUnlocked) touchActivity().catch(() => {});
    };

    window.addEventListener('mousemove', handleUserActivity);
    window.addEventListener('keydown', handleUserActivity);
    window.addEventListener('click', handleUserActivity);

    const interval = setInterval(async () => {
      try {
        const unlocked = await checkVaultStatus();
        setIsUnlocked(unlocked);
      } catch {
        setIsUnlocked(false);
      }
    }, 1000);

    return () => {
      window.removeEventListener('mousemove', handleUserActivity);
      window.removeEventListener('keydown', handleUserActivity);
      window.removeEventListener('click', handleUserActivity);
      clearInterval(interval);
    };
  }, [isUnlocked]);

  const handleUnlock = async (passphrase: string) => {
    await unlockVault(passphrase);
    setIsUnlocked(true);
  };

  const handleLock = async () => {
    await lockVault();
    setIsUnlocked(false);
  };

  return (
    <div className="flex h-screen w-screen flex-col bg-zinc-950 text-zinc-200 select-none overflow-hidden">
      <header className="drag-region flex h-9 w-full items-center justify-between border-b border-zinc-800/60 bg-zinc-900/50 px-3 text-xs text-zinc-400">
        <span className="font-mono font-medium tracking-wide text-zinc-300">MICORE</span>
        <div className="no-drag flex items-center gap-2">
          <span className={`h-2 w-2 rounded-full ${isUnlocked ? 'bg-emerald-500' : 'bg-amber-500'}`} />
          <span className="text-[10px] text-zinc-500">{isUnlocked ? 'UNLOCKED' : 'LOCKED'}</span>
        </div>
      </header>

      <main className="flex flex-1 overflow-hidden">
        {!isUnlocked ? (
          <UnlockPage onUnlock={handleUnlock} />
        ) : (
          <DashboardPage onLock={handleLock} />
        )}
      </main>
    </div>
  );
}