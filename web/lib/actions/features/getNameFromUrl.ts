export const getNameFromUrl = (url: string): string => {
  const array = url.split("/");
  const name = array[array.length - 1].split(".")[0];
  return name;
};
