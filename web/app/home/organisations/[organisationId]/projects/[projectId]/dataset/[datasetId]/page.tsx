"use client";
import { Button } from "@/components/ui/Button";
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
  useEffect(() => {
    async function getDataset() {
      setLoading(true);
      try {
        const file: AxiosResponse<File> = await axios.get(
          `/api/organisations/${organisationId}/projects/${projectId}/datasets/${datasetId}`,
        );
        setDataset(file.data);
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
      <div className="flex flex-row justify-between ml-auto md:w-[60%] mb-2">
        {/* select feature */}
        <Select
          onValueChange={(value) =>
            handleFeatureChange({
              label: "organisation",
              value,
            })
          }
        >
          <SelectTrigger className="w-[300px]">
            <SelectValue placeholder="Sql" />
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
      <Feature feature={selectedFeature} dataset={dataset!} />
    </main>
  );
}
