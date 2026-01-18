interface ComingSoonViewProps {
  title: string;
  description: string;
}

const ComingSoonView: React.FC<ComingSoonViewProps> = ({
  title,
  description,
}) => {
  return (
    <div className="h-full p-6 flex items-center justify-center">
      <div className="text-center">
        <h1 className="text-2xl font-bold text-white mb-2">{title}</h1>
        <p className="text-gray-400 mb-6">{description}</p>
        <div className="bg-gray-800 px-4 py-2 rounded-lg border border-gray-700 inline-block">
          <span className="text-yellow-500 text-sm">In Development</span>
        </div>
      </div>
    </div>
  );
};

export default ComingSoonView;
