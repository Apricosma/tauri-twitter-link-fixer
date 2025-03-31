import { useState } from "react";
import sources from "../converter-sources.json";
import Card from "../Components/Card";
import ContentContainer from "../Components/ContentContainer";

const TwitterContent = () => {
  const twitterSources = sources.sources.find(
    (source) => source.platform === "twitter"
  );

  const [selectedCard, setSelectedCard] = useState<string | null>(null);

  const handleCardClick = (embed: string) => {
    setSelectedCard((prev) => (prev === embed ? null : embed));
  };

  return (
    <>
      <h1 className="text-4xl flex items-center justify-center">Twitter Embeds</h1>

      <div className="flex items-center justify-center space-x-6">
        <img
          alt="Twitter Link Fixer Logo"
          className="h-24 w-24 drop-shadow-xl hover:animate-spin"
          src="/twitter.svg"
        />
      </div>
      <ContentContainer>
        {twitterSources?.embeds.map((embed, index) => (
          <Card
            key={index}
            embed={embed}
            isSelected={selectedCard === embed}
            onClick={() => handleCardClick(embed)}
          />
        ))}
      </ContentContainer>
    </>
  );
};

export default TwitterContent;