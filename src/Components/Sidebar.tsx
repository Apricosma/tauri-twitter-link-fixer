import React from "react";

interface SidebarProps {
  items: { label: string; onClick: () => void }[];
}

const Sidebar: React.FC<SidebarProps> = ({ items }) => {
  return (
    <aside className="fixed top-0 left-0 w-64 h-full bg-gray-900 text-white shadow-lg">
      <div className="p-4 text-lg font-bold border-b border-gray-700">
        Sidebar
      </div>
      <ul className="mt-4 space-y-2">
        {items.map((item, index) => (
          <li key={index}>
            <button
              onClick={item.onClick}
              className="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none"
            >
              {item.label}
            </button>
          </li>
        ))}
      </ul>
    </aside>
  );
};

export default Sidebar;