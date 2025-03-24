"use client";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";
import { useSearchParams } from "next/navigation";

export default function Page() {
  const query = useSearchParams();
  const token = query.get("token");

  return (
    <Tabs defaultValue="upload" className="w-full h-full p-6">
      <TabsList className="grid w-full grid-cols-2 w-[400px] mx-auto">
        <TabsTrigger value="upload">Upload</TabsTrigger>
        <TabsTrigger value="download">Download</TabsTrigger>
      </TabsList>
      <TabsContent value="upload">
        <UploadScreen />
        <p>{token}</p>
      </TabsContent>
      <TabsContent value="download">
        <DownloadScreen />
        <p>{token}</p>
      </TabsContent>
    </Tabs>
  );
}


function UploadScreen() {

  return (
    <Card className="h-full">
      upload
      <QRCode />
    </Card>
  );
}


function DownloadScreen() {

  return (
    <Card className="h-full">
      download
    </Card>
  );
}


function QRCode() {
  return (
    <div className="p-2">
      <Button size="sm">
        <p className="text-xs">Regenerate QR Code</p>
      </Button>
    </div>
  );
}
