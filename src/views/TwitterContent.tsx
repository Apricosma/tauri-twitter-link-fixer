import sources from "../converter-sources.json";

const TwitterContent = () => {
  const twitterSources = sources.sources.find(
    (source) => source.platform === "twitter"
  );

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center"></h1>

      <div className="flex items-center justify-center space-x-6">
        <img
          alt="Twitter Link Fixer Logo"
          className="h-24 w-24 drop-shadow-xl hover:animate-spin"
          src="/twitter.svg"
        />
      </div>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 p-4">
        {twitterSources?.embeds.map((embed, index) => (
          <div
            key={index}
            className="h-24 w-full bg-appforeground flex items-center justify-center shadow-md hover:bg-gray-300 rounded-md"
          >
            {embed}
          </div>
        ))}
      </div>
    </>
  );
};

export default TwitterContent;
