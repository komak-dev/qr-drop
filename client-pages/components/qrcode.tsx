import { QRCodeSVG } from "qrcode.react";

export default function QRCode({ value }: { value: string }) {
  return (
    <div className="h-full content-center">
      <QRCodeSVG value={value} size={108} />
    </div>
  );
}
