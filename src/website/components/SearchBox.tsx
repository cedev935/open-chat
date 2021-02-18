import React from "react";
import SearchIcon from "../assets/icons/search.svg";

export default React.memo(SearchBox);

type Props = {
    id: string,
    text: string,
    onChange: (text: string) => void,
    defaultPlaceholderText: string,
}

function SearchBox(props: Props) {
    return <div className="search">
        <input
            id={props.id}
            value={props.text}
            onChange={e => props.onChange(e.target.value)}
            placeholder={props.defaultPlaceholderText} />
        <SearchIcon />
    </div>;
}