"use client";
import { FC, useState } from "react";
import { Input } from "../dataEntry/input";
import Icons from "../icons";
interface SearchBarProps {
  placeholder: string;
  callback: (value: string) => void;
  withoutPrefixIcon?: boolean;
  inputClassName?: string;
  wrapperClassName?: string;
  value?: string;
}
export const SearchBar: FC<SearchBarProps> = ({
  placeholder,
  callback,
  withoutPrefixIcon,
  inputClassName,
  wrapperClassName,
  value: inputValue,
}) => {
  const [value, setSearchValue] = useState(inputValue);
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchValue(e.target.value);
    callback(e.target.value);
  };

  return (
    <div className={"relative " + wrapperClassName}>
      <Input
        type="text"
        className={"text-center " + inputClassName}
        placeholder={placeholder}
        prefix={withoutPrefixIcon ? null : <Icons.SearchIcon />}
        onChange={handleChange}
        value={value}
      />
    </div>
  );
};
