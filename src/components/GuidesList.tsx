import { useEffect, useState } from "preact/hooks";
import { getAllGuides } from "../api/api";
import { ListGuide } from "../api/types";
import Guide from "./Guide";

const GuideList = () => {
  const [guides, setGuides] = useState<ListGuide[]>([]);

  useEffect(() => {
    (async () => {
      const res = await getAllGuides();
      setGuides(res.guides);
    })();
  }, []);

  return <div>{guides.map((el) => Guide(el))}</div>;
};

export default GuideList;
