import { IoIosSearch } from "react-icons/io";
import "./style.css";
import { Button, Flex, Input, type InputRef } from "antd";
import { useEffect, useRef, useState } from "react";
import { TbLogout } from "react-icons/tb";

interface paramProps {
  searchValue: string,
  setSearchValue: (x: string) => void,
}

export default function NavBar({ searchValue, setSearchValue }: paramProps) {

  const [inputOpen, setInputOpen] = useState(false);
  const iconSize = 30;

  const inputRef = useRef<InputRef>(null);
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const inputElement = inputRef.current?.input;

      if (inputElement && !inputElement.contains(event.target as Node)) {
        setInputOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const openSearchInput = () => {
    setInputOpen(true);
    inputRef.current?.focus();
  }

  return (
    <nav className="NavBar">
      <Flex justify="end">

        <Flex gap={10} className="NavBar-search" align="center" justify="end">
          {!inputOpen && <Button
            className="NavBar-search-btn"
            type="text"
            onClick={() => openSearchInput()}
            icon={<IoIosSearch size={iconSize} />}
          />}
          <Input
            ref={inputRef}
            className={`NavBar-search-input ${inputOpen ? "open" : ""}`}
            placeholder="Busca un anime..."
            value={searchValue}
            onChange={(e) => setSearchValue(e.target.value)}
          />
        </Flex>
        <Button type="text" icon={<TbLogout size={iconSize} />} />
      </Flex>
    </nav>
  )
}
