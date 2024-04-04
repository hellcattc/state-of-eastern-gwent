import { ListGuide } from "../api/types";

const Guide = (guideData: ListGuide) => {
  return (
    <>
      <a
        href={`https://www.playgwent.com/en/decks/guides/${guideData.id}`}
        target="_blank"
      >
        <div className="p-5 my-10 border border-black rounded">
          <p>{guideData.id}</p>
          <p>{guideData.created}</p>
          <p className="mt-3">{guideData.name}</p>
        </div>
      </a>
    </>
  );
};

export default Guide;
