import { useEffect } from "react";
import { toast } from "sonner";

interface ConversionEvent {
  original: string;
  converted: string;
}

interface ConversionNotificationProps {
  show: boolean;
  conversion: ConversionEvent | null;
}

const ConversionNotification = ({ show, conversion }: ConversionNotificationProps) => {
  useEffect(() => {
    if (show && conversion) {
      try {
        const originalHost = new URL(conversion.original).hostname;
        const convertedHost = new URL(conversion.converted).hostname;
        
        toast.success("Link Converted!", {
          description: `${originalHost} → ${convertedHost}`,
          duration: 3000,
          style: {
            background: "rgba(34, 197, 94, 0.8)",
            color: "hsl(var(--foreground))",
            border: "1px solid hsl(var(--border))",
          },
        });
      } catch (error) {
        // Fallback if URL parsing fails
        toast.success("Link Converted!", {
          description: "Link has been successfully converted",
          duration: 3000,
          style: {
            background: "rgba(39, 39, 42, 0.8)",
            color: "hsl(var(--foreground))",
            border: "1px solid hsl(var(--border))",
          },
        });
      }
    }
  }, [show, conversion]);

  return null;
};

export default ConversionNotification;