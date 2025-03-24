import { useState, SetStateAction, Dispatch } from "react";
import { FileWithPath } from "react-dropzone";
import {
  FaFile, FaFilePdf, FaFileWord, FaFileExcel, FaFileImage, FaFileCode, FaFileAlt, FaFileArchive,
  FaFileAudio, FaFileVideo, FaFilePowerpoint
} from "react-icons/fa";
import { XIcon } from "lucide-react";


const extensionToIcon: { [key: string]: { icon: React.ElementType, color: string } } = {
  // Document files
  pdf: { icon: FaFilePdf, color: "#E74C3C" },
  doc: { icon: FaFileWord, color: "#3498DB" },
  docx: { icon: FaFileWord, color: "#3498DB" },
  xls: { icon: FaFileExcel, color: "#2ECC71" },
  xlsx: { icon: FaFileExcel, color: "#2ECC71" },
  ppt: { icon: FaFilePowerpoint, color: "#E67E22" },
  pptx: { icon: FaFilePowerpoint, color: "#E67E22" },
  txt: { icon: FaFileAlt, color: "#95A5A6" },

  // Image files
  png: { icon: FaFileImage, color: "#F1C40F" },
  jpg: { icon: FaFileImage, color: "#F1C40F" },
  jpeg: { icon: FaFileImage, color: "#F1C40F" },
  gif: { icon: FaFileImage, color: "#F1C40F" },
  svg: { icon: FaFileImage, color: "#F1C40F" },
  bmp: { icon: FaFileImage, color: "#F1C40F" },

  // Archive files
  zip: { icon: FaFileArchive, color: "#E67E22" },
  rar: { icon: FaFileArchive, color: "#E67E22" },
  tar: { icon: FaFileArchive, color: "#E67E22" },
  gz: { icon: FaFileArchive, color: "#E67E22" },
  "7z": { icon: FaFileArchive, color: "#E67E22" },

  // Code files
  js: { icon: FaFileCode, color: "#F39C12" },
  jsx: { icon: FaFileCode, color: "#8E44AD" },
  ts: { icon: FaFileCode, color: "#2980B9" },
  tsx: { icon: FaFileCode, color: "#9B59B6" },
  html: { icon: FaFileCode, color: "#E44D26" },
  css: { icon: FaFileCode, color: "#3498DB" },
  json: { icon: FaFileCode, color: "#16A085" },
  py: { icon: FaFileCode, color: "#3572A5" },
  java: { icon: FaFileCode, color: "#B07219" },
  cpp: { icon: FaFileCode, color: "#00599C" },
  c: { icon: FaFileCode, color: "#555555" },
  go: { icon: FaFileCode, color: "#00ADD8" },
  rs: { icon: FaFileCode, color: "#DEA584" },

  // Audio files
  mp3: { icon: FaFileAudio, color: "#8E44AD" },
  wav: { icon: FaFileAudio, color: "#8E44AD" },
  m4a: { icon: FaFileAudio, color: "#8E44AD" },
  flac: { icon: FaFileAudio, color: "#8E44AD" },

  // Video files
  mp4: { icon: FaFileVideo, color: "#2980B9" },
  mkv: { icon: FaFileVideo, color: "#2980B9" },
  avi: { icon: FaFileVideo, color: "#2980B9" },
  mov: { icon: FaFileVideo, color: "#2980B9" },
  wmv: { icon: FaFileVideo, color: "#2980B9" },
  flv: { icon: FaFileVideo, color: "#2980B9" },
};


type FileItemProps = {
  name: string;
  setFiles: Dispatch<SetStateAction<FileWithPath[]>>;
};

export function FileItem({ name, setFiles }: FileItemProps) {
  const extension = (name?.split(".").pop() || "").toLowerCase();

  const { icon: Icon, color } = extensionToIcon[extension] || FaFile;

  const [isHovered, setIsHovered] = useState(false);

  function removeThisFile() {
    setFiles(prevFiles => prevFiles.filter(f => f.name !== name));
  }

  return (
    <div 
      className="z-100 items-center gap-2 w-14 p-1 pb-2 mx-auto pointer-events-auto relative w-18"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <Icon color={color} size={52} className="mx-auto" />
      <p className="text-xs truncate pt-1">{name}</p>
      {
        isHovered && 
          <XIcon 
            size={18} 
            className="absolute top-0 left-2 p-[1px] bg-white border-1 border-slate-500 rounded-[10px]" 
            onClick={removeThisFile}
          />
      }
    </div>
  );
}
