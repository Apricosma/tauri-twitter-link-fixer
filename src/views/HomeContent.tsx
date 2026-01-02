import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../components/ui/card";

const HomeContent = () => {
  return (
    <div className="space-y-6 p-6 bg-sidebar rounded-2xl h-full min-h-full rounded-bl-none rounded-br-none">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">
          Welcome to Link Fixer
        </h1>
        <p className="text-muted-foreground">
          Convert social media links to embeddable formats automatically.
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <Card className="p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🐦</span>
            <h3 className="font-semibold">Twitter/X</h3>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🦋</span>
            <h3 className="font-semibold">BlueSky</h3>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">🦋</span>
            <h3 className="font-semibold">Instagram</h3>
          </div>
        </Card>

        <Card className="p-4">
          <div className="flex items-center space-x-2">
            <span className="text-2xl">⚡</span>
            <h3 className="font-semibold">Add your own!</h3>
          </div>
        </Card>
      </div>

      <Card className="gap-4 pt-4">
        <CardHeader className="gap-0">
          <CardTitle className="text-xl">Getting Started</CardTitle>
        </CardHeader>
        <CardContent>
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
              <p className="text-sm">
                Toggle the platform on to enable conversion
              </p>
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
              <p className="text-sm">
                Copy any supported link - it will be automatically converted!
              </p>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default HomeContent;
