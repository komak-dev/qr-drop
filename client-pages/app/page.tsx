"use client";

import { Card, CardContent } from "@/components/ui/card";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";
import { useSearchParams } from "next/navigation";
import { useState, useEffect } from "react";
import { FileWithPath } from "react-dropzone";
import QRCode from "@/components/qrcode";
import UploadScreen from "@/components/upload-screen";
import DownloadScreen from "@/components/download-screen";


export default function Page() {
  const query = useSearchParams();
  const token = query.get("token") || "";
  const apiUrl = query.get("apiUrl") || "";

  const [origin, setOrigin] = useState("");

  const [uploadFiles, setUploadFiles] = useState<FileWithPath[]>([]);

  useEffect(() => {
    setOrigin(window.location.origin);
  }, []);


  return (
    <Tabs defaultValue="upload" className="w-full h-full p-6">
      <TabsList className="grid w-full grid-cols-2 w-[400px] mx-auto">
        <TabsTrigger value="upload">Upload</TabsTrigger>
        <TabsTrigger value="download">Download</TabsTrigger>
      </TabsList>
      <TabsContent value="upload">
        <Card className="h-full">
          <CardContent className="h-full flex gap-6">
            <QRCode value={`${origin}/d2m?token=${token}&apiUrl=${apiUrl}`} />
            <UploadScreen files={uploadFiles} setFiles={setUploadFiles} />
          </CardContent>
        </Card>
      </TabsContent>
      <TabsContent value="download">
        <Card className="h-full">
          <CardContent className="h-full flex gap-6">
            <QRCode value={`${origin}/m2d?token=${token}&apiUrl=${apiUrl}`} />
            <DownloadScreen />
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
  );
}

