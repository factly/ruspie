'use client';
import { FC } from "react";
import { Input } from "../dataEntry/input";
import Icons from "../icons";
interface SearchBarProps {
  placeholder: string;
  callback: () => void;
}
export const SearchBar: FC<SearchBarProps> = ({ placeholder, callback }) => {
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    callback();
  };

  return (
    <div>
      <Input
        type="text"
        className="text-center"
        placeholder={placeholder}
        prefix={<Icons.SearchIcon />}
        onChange={handleChange}
      />
    </div>
  );
};
