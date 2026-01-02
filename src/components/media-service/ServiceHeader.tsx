interface ServiceHeaderProps {
  icon: string;
  title: string;
  description: string;
}

const ServiceHeader: React.FC<ServiceHeaderProps> = ({
  title,
  description,
}) => {
  return (
    <div className="flex items-center space-x-3">
      <div>
        <h1 className="text-3xl font-bold">{title}</h1>
        <p className="text-muted-foreground">{description}</p>
      </div>
    </div>
  );
};

export default ServiceHeader;
