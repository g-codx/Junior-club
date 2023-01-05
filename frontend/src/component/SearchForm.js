import {useState} from "react";


function SearchFormComponent() {

    const [selected, setSelected] = useState("");

    return (
        <div className="search-container p-2">
            <label className="text-color p-2" htmlFor="tag">Search tag</label>

            <select
                className="border-lite-slim background-color text-color"
                name="tag"
                id="tag"
                aria-label="tag"
                value={selected}
                onChange={e => setSelected(e.target.value)}
            >
                <option>All</option>
                <option>Java</option>
                <option>Rust</option>
            </select>
        </div>
    )
}


export default SearchFormComponent;