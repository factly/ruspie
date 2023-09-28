import React from "react";
import { FeatureType } from "@/types/feature";
import SearchSql from "./searchSql";
import Graphql from "./graphql";
import RestApi from "./restApi";
import Schema from "./schema";
import { File } from "@/types/file";

type Props = {
  feature: FeatureType;
  dataset: File;
};

function Feature({ feature, dataset }: Props) {
  if (feature.value === "SQL") {
    return <SearchSql />;
  }

  if (feature.value === "rest") {
    return <RestApi dataset={dataset} />;
  }

  if (feature.value === "graphql") {
    return <Graphql />;
  }

  if (feature.value === "schema") {
    return <Schema />;
  }

  return <h2>Feature</h2>;
}

export default Feature;
