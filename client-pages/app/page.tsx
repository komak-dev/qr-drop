import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs"

export default function Page() {
  return (
    <Tabs defaultValue="upload" className="w-full h-full p-6">
      <TabsList className="grid w-full grid-cols-2 w-[400px] mx-auto">
        <TabsTrigger value="upload">Upload</TabsTrigger>
        <TabsTrigger value="download">Download</TabsTrigger>
      </TabsList>
      <TabsContent value="upload">
        <UploadScreen />
      </TabsContent>
      <TabsContent value="download">
        <DownloadScreen />
      </TabsContent>
    </Tabs>
  )
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
