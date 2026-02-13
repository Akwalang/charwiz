import React from "react";

import { Art, ArtName } from "@/views/components";

interface ResponseProps {}

export const Response: React.FC<ResponseProps> = () => {
  return (
    <div className="flex w-full h-full items-center justify-center">
      <Art
        name={ArtName.QueenFirstMeet}
        className="w-[70%] min-w-[320px] max-w-[540px]"
        ratio={1.8}
      />
    </div>
  );
};
