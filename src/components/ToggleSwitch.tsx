import React from "react";

interface ToggleSwitchProps {
  id: string;
  label?: string;
  platform: string;
  initialChecked: boolean;
  disabled?: boolean;
  className?: string;
  onToggle?: (enabled: boolean) => void;
}

export const ToggleSwitch: React.FC<ToggleSwitchProps> = ({
  id,
  label,
  initialChecked,
  disabled = false,
  className = "",
  onToggle,
}) => {
  const handleToggle = async () => {
    if (disabled) return;

    try {
      await onToggle?.(!initialChecked);
    } catch (error) {
      console.error("Failed to toggle:", error);
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
        aria-checked={initialChecked}
        disabled={disabled}
        onClick={handleToggle}
        className={`w-12 h-6 flex items-center rounded-full p-1 transition-colors duration-150 border-1 border-appbg hover:border-blue-200
          ${initialChecked ? "bg-blue-600" : "bg-appbg"}
          ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
      >
        <div
          className={`bg-white w-4 h-4 rounded-full shadow-md transform transition-transform duration-150
            ${initialChecked ? "translate-x-6" : "translate-x-0"}`}
        />
      </button>
    </div>
  );
};

export default ToggleSwitch;