import { Suspense } from "react";
import { ViewType } from "../components/ActiveViewContent";
import { useConfig } from "../hooks/useConfig";
import { getSimpleIcon } from "../utils/iconMapper";
import { HelpCircle, CircleAlert, Car } from "lucide-react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "../components/ui/card";

interface HomeContentProps {
  setActiveView: (view: ViewType) => void;
}

const HomeContent = ({ setActiveView }: HomeContentProps) => {
  const { config, loading } = useConfig();

  if (loading || !config) {
    return (
      <div className="space-y-6 p-6 bg-sidebar rounded-2xl h-full min-h-full rounded-bl-none rounded-br-none">
        <div className="text-center text-muted-foreground">Loading...</div>
      </div>
    );
  }
  return (
    <div className="space-y-6 p-6 bg-sidebar rounded-2xl h-full min-h-full rounded-bl-none rounded-br-none">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">
          Welcome to Cosma Converter
        </h1>
        <p className="text-muted-foreground">
          Your one-stop app to convert social media links to embeddable formats
          for Discord and more.
        </p>
      </div>

      <div className="grid gap-4 md:grid-cols-3 lg:grid-cols-3">
        {config.sources.map((source) => {
          const Icon = getSimpleIcon(source.metadata.icon);
          const view = source.platform;

          return (
            <Card
              key={source.platform}
              className="flex items-center p-4 hover:border-primary transition-colors cursor-pointer relative"
              onClick={() => setActiveView(view)}
            >
              <div className="flex items-center space-x-2">
                <Suspense
                  fallback={<HelpCircle size={48} className="shrink-0 block" />}
                >
                  <Icon size={48} className="shrink-0 block" />
                </Suspense>
              </div>
            </Card>
          );
        })}

        <Card className="p-2 flex items-center justify-center border-dashed border-2 hover:border-primary transition-colors cursor-pointer">
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
      <Card className="gap-2">
        <CardHeader className="text-xl">
          <CardTitle className="flex gap-2 items-center">
            <CircleAlert size={32} className="text-primary" />
            Note
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-y-4">
            <p>
              All included embedding services
              <span className="text-primary"> are not</span> affiliated with
              Cosma Converter. We have no control over their availability or
              usage.
            </p>
            <p>
              These services may or may not work, and their availability can
              change without notice. If a service stops working for more than a
              week, please notify us via GitHub Issues.
            </p>
            <p>Please use these services at your own discretion.</p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default HomeContent;
