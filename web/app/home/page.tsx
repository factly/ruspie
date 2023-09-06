"use client";
import { Organisation } from "../../components/ui/Organisation";
import { SearchBar } from "@/components/ui/SearchBar";
import { Organisation as Org } from "@/types/organisation";

const organisations: Org[] = [
  {
    id: "hell",
    title: "hell",
    createdAt: "hell",
    updatedAt: "hello",
  },
];
const Page = () => {
  return (
    <div>
      <div className="flex justify-evenly">
        <div className="w-full m-4 font-semibold text-lg">Organisations</div>
        <div className="w-full mt-10 flex flex-col relative gap-10">
          <div className="w-full">
            <SearchBar placeholder="Search organisations" callback={() => { }} />
          </div>
          <div className="mt-5 container flex flex-col gap-10">
            {organisations.map((org, index) => (
              <div key={index}>
                <Organisation
                  id={org.id}
                  title={org.title}
                  createdAt=""
                  updatedAt=""
                />
              </div>
            ))}
          </div>
        </div>
        <div className="w-full m-4 text-right">Organisations</div>
      </div>
    </div>
  );
};

export default Page;
