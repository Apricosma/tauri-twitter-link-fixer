import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface ToggleSwitchProps {
  id: string;
  label?: string;
  platform: string; // Platform name (e.g., "twitter", "bluesky")
  initialChecked?: boolean; // Initial state of the toggle
  disabled?: boolean;
  className?: string;
  onToggle?: (enabled: boolean) => void; // Callback for toggle state changes
}

export const ToggleSwitch: React.FC<ToggleSwitchProps> = ({
  id,
  label,
  platform,
  initialChecked = false,
  disabled = false,
  className = "",
  onToggle,
}) => {
  const [checked, setChecked] = useState(initialChecked);

  const handleToggle = async () => {
    if (disabled) return;

    const newChecked = !checked;
    setChecked(newChecked);

    try {
      // Call the backend to update the platform's enabled state
      await invoke("toggle_platform", { platform, enabled: newChecked });
      console.log(`${platform} is now ${newChecked ? "enabled" : "disabled"}`);
      // Call the onToggle callback if provided
      onToggle?.(newChecked);
    } catch (error) {
      console.error("Failed to toggle platform:", error);
      // Revert the toggle state in case of an error
      setChecked(!newChecked);
    }
  };

  return (
    <div className={`flex items-center justify-between ${className}`}>
      {label && (
        <label htmlFor={id} className="text-md font-bold">
          {label}
        </label>
      )}
      <button
        id={id}
        type="button"
        role="switch"
        aria-checked={checked}
        disabled={disabled}
        onClick={handleToggle}
        className={`w-12 h-6 flex items-center rounded-full p-1 transition-colors duration-150 border-1 border-appbg hover:border-blue-200
          ${checked ? "bg-blue-600" : "bg-appbg"}
          ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
      >
        <div
          className={`bg-white w-4 h-4 rounded-full shadow-md transform transition-transform duration-150
            ${checked ? "translate-x-6" : "translate-x-0"}`}
        />
      </button>
    </div>
  );
};

export default ToggleSwitch;