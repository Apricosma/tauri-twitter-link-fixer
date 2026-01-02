const HomeContent = () => {
  return (
    <div className="space-y-6 p-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Welcome to Link Fixer</h1>
        <p className="text-muted-foreground">
          Convert social media links to embeddable formats automatically.
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🐦</span>
            <h3 className="font-semibold">Twitter/X</h3>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            Convert Twitter and X links to embeddable formats like fxtwitter and vxtwitter.
          </p>
        </div>

        <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🦋</span>
            <h3 className="font-semibold">BlueSky</h3>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            Transform BlueSky post links to embeddable formats for better sharing.
          </p>
        </div>

        <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">⚡</span>
            <h3 className="font-semibold">Automatic</h3>
          </div>
          <p className="text-sm text-muted-foreground mt-2">
            Monitors your clipboard and converts links automatically in the background.
          </p>
        </div>
      </div>

      <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
        <h3 className="font-semibold mb-4">Getting Started</h3>
        <div className="space-y-3">
          <div className="flex items-start space-x-3">
            <div className="flex-shrink-0 w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium">
              1
            </div>
            <p className="text-sm">Navigate to a platform from the sidebar</p>
          </div>
          <div className="flex items-start space-x-3">
            <div className="flex-shrink-0 w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium">
              2
            </div>
            <p className="text-sm">Toggle the platform on to enable conversion</p>
          </div>
          <div className="flex items-start space-x-3">
            <div className="flex-shrink-0 w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium">
              3
            </div>
            <p className="text-sm">Select your preferred converter service</p>
          </div>
          <div className="flex items-start space-x-3">
            <div className="flex-shrink-0 w-6 h-6 bg-primary text-primary-foreground rounded-full flex items-center justify-center text-sm font-medium">
              4
            </div>
            <p className="text-sm">Copy any supported link - it will be automatically converted!</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default HomeContent;