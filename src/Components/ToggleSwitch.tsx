import React, { useState } from "react";

interface ToggleSwitchProps {
  id: string;
  label?: string;
  checked?: boolean;
  disabled?: boolean;
  onChange?: (checked: boolean) => void;
  className?: string;
}

export const ToggleSwitch: React.FC<ToggleSwitchProps> = ({
  id,
  label,
  checked = false,
  disabled = false,
  onChange,
  className = "",
}) => {
  const [isChecked, setIsChecked] = useState<boolean>(checked);

  const handleToggle = () => {
    if (disabled) return;

    const newChecked = !isChecked;
    setIsChecked(newChecked);
    onChange?.(newChecked);
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
        aria-checked={isChecked}
        disabled={disabled}
        onClick={handleToggle}
        className={`w-12 h-6 flex items-center rounded-full p-1 transition-colors duration-150 border-1 border-appbg hover:border-blue-200
          ${isChecked ? "bg-blue-600" : "bg-appbg"}
          ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`}
      >
        <div
          className={`bg-white w-4 h-4 rounded-full shadow-md transform transition-transform duration-150
            ${isChecked ? "translate-x-6" : "translate-x-0"}`}
        />
      </button>
    </div>
  );
};

export default ToggleSwitch;
