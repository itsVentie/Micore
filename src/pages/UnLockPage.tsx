import { useState } from 'preact/hooks';

interface UnlockPageProps {
  onUnlock: (passphrase: string) => Promise<void>;
}

export function UnlockPage({ onUnlock }: UnlockPageProps) {
  const [passphrase, setPassphrase] = useState<string>('');
  const [error, setError] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    if (!passphrase) return;

    setLoading(true);
    setError('');

    try {
      await onUnlock(passphrase);
      setPassphrase('');
    } catch {
      setError('Invalid passphrase or vault key failure');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-1 items-center justify-center p-6">
      <form onSubmit={handleSubmit} className="w-full max-w-sm rounded-xl border border-zinc-800/80 bg-zinc-900/40 p-6 shadow-2xl backdrop-blur-sm">
        <h2 className="text-lg font-bold text-zinc-100 mb-1">Unlock Vault</h2>
        <p className="text-xs text-zinc-400 mb-4">Enter master passphrase to derive decryption key.</p>

        {error && (
          <div className="mb-3 rounded-lg bg-rose-500/10 border border-rose-500/20 p-2 text-xs text-rose-400">
            {error}
          </div>
        )}

        <input
          type="password"
          value={passphrase}
          onInput={(e) => setPassphrase((e.target as HTMLInputElement).value)}
          placeholder="Master Passphrase"
          className="w-full rounded-lg border border-zinc-700/60 bg-zinc-950/80 px-3 py-2 text-sm text-zinc-100 placeholder-zinc-500 focus:border-indigo-500 focus:outline-none mb-4"
          autoFocus
        />

        <button
          type="submit"
          disabled={loading}
          className="w-full rounded-lg bg-indigo-600 py-2 text-xs font-semibold text-white shadow-lg shadow-indigo-600/20 hover:bg-indigo-500 disabled:opacity-50 transition-all"
        >
          {loading ? 'Deriving Argon2id Key...' : 'Unlock Session'}
        </button>
      </form>
    </div>
  );
}