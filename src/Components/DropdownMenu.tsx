import React, { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

interface DropdownProps {
  options: string[];
  label?: string;
  selected?: string | null;
  platform: string; // Platform name (e.g., "twitter", "bluesky")
  onSelect?: (selected: string) => void; // Optional callback for additional actions
}

const DropdownMenu: React.FC<DropdownProps> = ({
  options,
  label,
  selected,
  platform,
  onSelect,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [currentSelection, setCurrentSelection] = useState<string | null>(selected || null);
  const dropdownRef = useRef<HTMLDivElement>(null);

  const handleToggle = () => {
    setIsOpen((prev) => !prev);
  };

  const handleSelect = (option: string) => {
    setCurrentSelection(option);
    setIsOpen(false);
  
    onSelect?.(option);
  };

  const handleClickOutside = (event: MouseEvent) => {
    if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
      setIsOpen(false);
    }
  };

  useEffect(() => {
    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <div ref={dropdownRef} className="flex items-center justify-between space-x-4">
      {label && (
        <label className="text-sm font-medium text-gray-300">
          {label}
        </label>
      )}
      <div className="relative inline-block text-left w-48">
        <button
          type="button"
          onClick={handleToggle}
          className="inline-flex justify-between w-full rounded-md border border-gray-600 shadow-sm px-4 py-2 bg-appbg text-sm font-medium text-gray-300 hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
        >
          {currentSelection || "Select an option"}
          <svg
            className={`-mr-1 ml-2 h-5 w-5 transform transition-transform ${
              isOpen ? "rotate-180" : "rotate-0"
            } text-gray-300`}
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 20 20"
            fill="currentColor"
            aria-hidden="true"
          >
            <path
              fillRule="evenodd"
              d="M5.23 7.21a.75.75 0 011.06.02L10 10.94l3.71-3.71a.75.75 0 111.06 1.06l-4 4a.75.75 0 01-1.06 0l-4-4a.75.75 0 01.02-1.06z"
              clipRule="evenodd"
            />
          </svg>
        </button>
        {isOpen && (
          <div className="absolute z-10 mt-2 w-full rounded-md bg-appforeground shadow-lg border border-gray-600">
            <ul className="py-1 text-sm text-gray-300">
              {options.map((option, index) => (
                <li
                  key={index}
                  onClick={() => handleSelect(option)}
                  className="cursor-pointer px-4 py-2 hover:bg-gray-700"
                >
                  {option}
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
};

export default DropdownMenu;