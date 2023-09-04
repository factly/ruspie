import { FC } from "react";

interface SearchBarProps {
  placeholder: string;
  callback: () => void;
}
export const SearchBar: FC<SearchBarProps> = ({ placeholder, callback }) => {
  return (
    <div className="relative">
      <input
        type="text"
        className="focus:border-black w-full h-8 py-2 pl-10 placeholder:text-center placeholder:mr-2 placeholder:text-md pr-4 rounded-lg border border-gray-300 focus:outline-none"
        placeholder={placeholder}
      />
      <button
        onClick={() => callback()}
        className="absolute inset-y-0 left-0 flex items-center pl-3"
      >
        <svg
          width="20"
          height="20"
          viewBox="0 0 20 20"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M9.16667 15.8333C12.8486 15.8333 15.8333 12.8486 15.8333 9.16667C15.8333 5.48477 12.8486 2.5 9.16667 2.5C5.48477 2.5 2.5 5.48477 2.5 9.16667C2.5 12.8486 5.48477 15.8333 9.16667 15.8333Z"
            stroke="black"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
          <path
            d="M17.5 17.5L13.875 13.875"
            stroke="black"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    </div>
  );
};
