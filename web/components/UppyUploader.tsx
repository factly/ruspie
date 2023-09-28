"use client";
import { Uppy, UppyFile } from "@uppy/core";
import AwsS3 from "@uppy/aws-s3";
import "@uppy/core/dist/style.css";
import "@uppy/dashboard/dist/style.css";
import { Dashboard } from "@uppy/react";

function UppyUploader({
  onUpload,
  isDataset,
}: {
  onUpload: (data: UppyFile, path: string) => void;
  isDataset: boolean;
}) {
  const allowedFileTypes = isDataset ? ["text/csv"] : ["image/*"];
  const maxFileSize = (isDataset ? 100 : 5) * 1024 * 1024;
  let path = "";
  const uppy = new Uppy({
    id: "uppy-media",
    restrictions: {
      maxNumberOfFiles: 1,
      minNumberOfFiles: 0,
      maxFileSize,
      allowedFileTypes,
    },
    autoProceed: false,
    allowMultipleUploads: false,
    onBeforeUpload: (files) => {
      const updatedFiles: { [key: string]: any } = {};
      Object.keys(files).forEach((fileID) => {
        const name = convertToAlphanumeric(files[fileID].name);
        path =
          (!isDataset
            ? "/" + new Date().getFullYear() + "/" + new Date().getMonth() + "/"
            : "ruspie_" + new Date().getTime()) +
          "_" +
          name;
        updatedFiles[fileID] = {
          ...files[fileID],
          file_name: name,
          meta: {
            ...files[fileID].meta,
            name: path,
          },
        };
      });
      return updatedFiles;
    },
  }).use(AwsS3, { companionUrl: "http://localhost:3020" });

  uppy.on("upload-success", (data) => {
    onUpload(data, path);
  });

  uppy.on("file-added", (file) => {
    const data = file.data;
    const url = data.thumbnail ? data.thumbnail : URL.createObjectURL(data);
    const image = new Image();
    image.src = url;
    image.onload = () => {
      uppy.setFileMeta(file.id, { width: image.width, height: image.height });
      URL.revokeObjectURL(url);
    };
    image.onerror = () => {
      URL.revokeObjectURL(url);
    };
  });

  return (
    <Dashboard
      className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8"
      uppy={uppy}
      plugins={["Url", "ImageEditor"]}
      metaFields={[
        { id: "name", name: "Name", placeholder: "file name" },
        {
          id: "caption",
          name: "Caption",
          placeholder: "describe what the image is about",
        },
        {
          id: "alt_text",
          name: "Alt Text",
          placeholder: "describe what the image is content",
        },
      ]}
    />
  );
}

function convertToAlphanumeric(str: string) {
  const alphanumericRegex = /[^a-zA-Z0-9]+/g;

  // Remove non-alphanumeric characters and return the result
  const result = str.replace(alphanumericRegex, "");

  return result;
}

export default UppyUploader;
