import { useCallback, SetStateAction, Dispatch } from "react";
import { useDropzone, FileWithPath } from "react-dropzone";


type Props = {
  files: FileWithPath[];
  setFiles: Dispatch<SetStateAction<FileWithPath[]>>;
};

export default function UploadScreen({files, setFiles}: Props) {

  const onDrop = useCallback((acceptedFiles: FileWithPath[]) => {
    acceptedFiles.forEach((file) => {
      if (files.find(f => f.path === file.path)) return;
      setFiles((prevFiles) => [...prevFiles, file]);
    });
  }, []);


  const { getRootProps, getInputProps, isDragActive } = useDropzone({ onDrop });


  return (
    <div className="border rounded-xl bg-slate-100 h-full w-full flex items-center justify-center text-slate-500 relative">
      <div {...getRootProps()} className="absolute top-0 left-0 right-0 bottom-0 flex items-center justify-center text-slate-500 z-50">
        <input {...getInputProps()} />
        {
          files.length == 0 && (
            isDragActive ?
              <p className="text-sm">Drop the files here ...</p> :
              <p className="text-sm">Drag 'n' drop some files here, or click to select files</p>
          )
        }
      </div>
      <div className="absolute top-0 left-0 right-0 bottom-0">
        <div className="grid grid-cols-5 py-4">
          {
            files.map((file) => (
              <div key={file.path} className="flex items-center gap-2">
                <FileItem name={file.name} setFiles={setFiles} />
              </div>
            ))
          }
        </div>
      </div>
    </div>
  );
}

