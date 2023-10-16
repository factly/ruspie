"use client";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useEffect, useState } from "react";
import { FeatureType } from "@/types/feature";
import Feature from "@/components/ui/feature";
import { File } from "@/types/file";
import axios, { AxiosResponse } from "axios";
import { Loader } from "lucide-react";
import { FileParam } from "@/types/params/file_param";
import Link from "next/link";
import { Project } from "@/types/organisation";
import { getBasepathUrl } from "@/lib/utils/baseUrl";

export default function Page({
  params: { organisationId, datasetId, projectId },
}: FileParam) {
  const features: FeatureType[] = [
    { label: "SQL", value: "SQL" },
    { label: "REST API", value: "rest" },
    { label: "GraphQL", value: "graphql" },
    { label: "Schema", value: "schema" },
  ];

  const [dataset, setDataset] = useState<File>();
  const [loading, setLoading] = useState<boolean>(false);
  const [project, setProject] = useState<Project>();
  const basePath = getBasepathUrl();
  useEffect(() => {
    async function getDataset() {
      setLoading(true);
      try {
        const file: AxiosResponse = await axios.get(
          basePath +
          `/api/organisations/${organisationId}/projects/${projectId}/datasets/${datasetId}`,
        );
        setDataset(file.data);
        console.log(file.data);
        setProject(file.data.project);
      } catch (err) {
      } finally {
        setLoading(false);
      }
    }
    getDataset();
  }, []);

  const [selectedFeature, setSelectedFeature] = useState(features[0]);

  const handleFeatureChange = (value: FeatureType) => {
    setSelectedFeature(value);
  };

  if (loading && !dataset) {
    return (
      <div className="h-screen flex items-center justify-center -mt-28">
        <Loader className="h-10 w-10 animate-spin text-gray-400" />
      </div>
    );
  }

  return (
    <main className="flex flex-col mt-10 bg-transparent px-8">
      <div className="flex flex-row items-center justify-between  md:w-[60%] mb-2">
        <div className="flex gap-3 items-center">
          <Link
            href={`/home/organisations/${organisationId}/projects/${projectId}`}
          >
            <h1 className="text-lg font-semibold text-gray-600 hover:text-black cursor-pointer">
              {project?.title} {"> "}
              {dataset?.name}{" "}
            </h1>
          </Link>
        </div>
        <div className="">
          <Select
            onValueChange={(value) =>
              handleFeatureChange({
                label: "organisation",
                value,
              })
            }
          >
            <SelectTrigger className="w-[300px]">
              <SelectValue placeholder="SQL" />
            </SelectTrigger>
            <SelectContent>
              {features.map(({ label, value }) => (
                <SelectItem value={value} key={value}>
                  {label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>
      <Feature feature={selectedFeature} dataset={dataset!} />
    </main>
  );
}
