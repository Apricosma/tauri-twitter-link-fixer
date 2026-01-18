import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { AppConfig } from "../types/config";

interface UseConfigReturn {
  config: AppConfig | null;
  loading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
}

/**
 * Hook to fetch and manage app configuration from Rust backend
 * Automatically listens for config updates via Tauri events
 */
export function useConfig(): UseConfigReturn {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchConfig = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const cfg = await invoke<AppConfig>("get_frontend_config");
      
      setConfig(cfg);
      setLoading(false);
    } catch (err) {
      setError(err as string);
      setLoading(false);
    }
  };

  // Load initial config on mount
  useEffect(() => {
    fetchConfig();
  }, []);

  // Listen for config changes from Rust backend
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      // TODO: Implement config-updated event in Rust
      unlisten = await listen<AppConfig>("config-updated", (event) => {
        setConfig(event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  return {
    config,
    loading,
    error,
    refetch: fetchConfig,
  };
}
