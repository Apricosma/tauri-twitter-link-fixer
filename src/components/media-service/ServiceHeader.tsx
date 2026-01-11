interface ServiceHeaderProps {
  icon?: React.ReactNode;
  title: string;
  description: string;
}

const ServiceHeader: React.FC<ServiceHeaderProps> = ({
  icon,
  title,
  description,
}) => {
  return (
    <div className="flex items-center justify-between space-x-3">
      <div>
        <h1 className="text-3xl font-bold">{title}</h1>
        <p className="text-muted-foreground">{description}</p>
      </div>
      {icon && <div className="flex-shrink-0">{icon}</div>}
    </div>
  );
};

export default ServiceHeader;
