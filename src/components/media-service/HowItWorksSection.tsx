interface HowItWorksSectionProps {
  steps: string[];
}

const HowItWorksSection: React.FC<HowItWorksSectionProps> = ({ steps }) => {
  return (
    <div className="rounded-lg border bg-card text-card-foreground shadow-sm p-6">
      <h3 className="font-semibold mb-3">How it works</h3>
      <div className="space-y-2 text-sm text-muted-foreground">
        {steps.map((step, index) => (
          <p key={index}>• {step}</p>
        ))}
      </div>
    </div>
  );
};

export default HowItWorksSection;