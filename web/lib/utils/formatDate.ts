import { format, parseISO } from "date-fns";

export const formatTimestamp = (timestamp: string) => {
  const date = parseISO(timestamp);
  const formattedDate = format(date, "dd MMM yyyy h:mm a ");
  return formattedDate;
};
