import { ListGuide } from "../api/types";

interface GuideProps {
  guideData: ListGuide
}

const Guide = ({guideData}: GuideProps) => {
  return (
    <>
      <a
        href={`https://www.playgwent.com/en/decks/guides/${guideData.id}`}
        target="_blank"
      >
        <div className="p-5 my-10 border border-black rounded">
          <p>ID: {guideData.id}</p>
          <p>Date: {guideData.created}</p>
          <p className="mt-3">{guideData.name}</p>
        </div>
      </a>
    </>
  );
};

export default Guide;
