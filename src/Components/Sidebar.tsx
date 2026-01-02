import React from "react";

interface SidebarProps {
  items: { label: string; onClick: () => void }[];
}

const Sidebar: React.FC<SidebarProps> = ({ items }) => {
  return (
    <div className="mt-5">
      <aside className="fixed left w-64 h-9/10 bg-background text-foreground border-r-1 border-gray-700/50">
        <div className="pb-4 text-lg font-bold border-b border-gray-700/50 w-9/10 mx-auto">
          Twitter Link Fixer
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
    </div>
  );
};

export default Sidebar;
