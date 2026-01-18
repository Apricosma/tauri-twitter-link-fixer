import React from "react";

interface CardProps {
  embed: string;
  isSelected: boolean;
  onClick: () => void;
}

const Card: React.FC<CardProps> = ({ embed, isSelected, onClick }) => {
  return (
    <div
      onClick={onClick}
      className={`h-24 w-full bg-appforeground flex items-center justify-center shadow-md rounded-md cursor-pointer ${
        isSelected ? "border-1 border-red-500" : "hover:bg-gray-500"
      }`}
    >
      {embed}
    </div>
  );
};

export default Card;
