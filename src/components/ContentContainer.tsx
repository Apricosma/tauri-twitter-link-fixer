import React from "react";

interface ContentContainerProps {
  children: React.ReactNode;
}

const ContentContainer: React.FC<ContentContainerProps> = ({ children }) => {
  return (
    <div className="grid grid-cols-1 gap-4 p-4 mt-4 max-w-150 mx-auto bg-appforeground rounded-lg shadow-lg">
      {children}
    </div>
  );
};

export default ContentContainer;
