'use client';
import { FC, useState } from "react";
import { Input } from "../dataEntry/input";
import Icons from "../icons";
interface SearchBarProps {
  placeholder: string;
  callback: (value: string) => void;
}
export const SearchBar: FC<SearchBarProps> = ({ placeholder, callback }) => {
  const [value, setSearchValue] = useState("")
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchValue(e.target.value);
    callback(e.target.value)
  };

  return (
    <div>
      <Input
        type="text"
        className="text-center"
        placeholder={placeholder}
        prefix={<Icons.SearchIcon />}
        onChange={handleChange}
        value={value}
      />
    </div>
  );
};
