import { Button, Flex } from "antd";
import { useRef } from "react";
import { FaAngleLeft, FaAngleRight } from "react-icons/fa6";
import "./style.css";

type Props = {
  children: React.ReactNode;
};

export default function Carousel({ children }: Props) {
  const carouselRef = useRef<HTMLDivElement>(null);

  const scroll = (direction: "left" | "right") => {
    if (!carouselRef.current) return;

    const scrollAmount = 300;

    carouselRef.current.scrollBy({
      left: direction === "left" ? -scrollAmount : scrollAmount,
      behavior: "smooth",
    });
  };

  return (
    <div className="Carousel">
      <Button
        type="text"
        icon={<FaAngleLeft size={30} color="#fff" />}
        className="Carousel-btn left"
        onClick={() => scroll("left")}
      />

      <div className="Carousel-scroll" ref={carouselRef}>
        <Flex gap={12} className="Carousel-track">
          {children}
        </Flex>
      </div>

      <Button
        type="text"
        icon={<FaAngleRight size={30} color="#fff" />}
        className="Carousel-btn right"
        onClick={() => scroll("right")}
      />
    </div>
  );
}
