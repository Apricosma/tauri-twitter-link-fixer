interface ConversionEvent {
  original: string;
  converted: string;
}

interface ConversionNotificationProps {
  show: boolean;
  conversion: ConversionEvent | null;
}

const ConversionNotification = ({ show, conversion }: ConversionNotificationProps) => {
  if (!show || !conversion) return null;

  return (
    <div className="fixed bottom-4 right-4 bg-appforeground text-gray-100 p-4 rounded-lg shadow-lg transition-opacity duration-300 max-w-2xl border border-gray-700">
      <div className="font-bold mb-2 text-red-400">Link Converted!</div>
      <div className="text-sm opacity-90 space-y-2">
        <div>
          <div className="font-semibold mb-1">From ({new URL(conversion.original).hostname}):</div>
          <div className="bg-appbg/50 p-2 rounded break-all">
            {conversion.original}
          </div>
        </div>
        <div>
          <div className="font-semibold mb-1">To ({new URL(conversion.converted).hostname}):</div>
          <div className="bg-appbg/50 p-2 rounded break-all">
            {conversion.converted}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConversionNotification;