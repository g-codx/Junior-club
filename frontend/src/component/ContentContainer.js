import CreatPostComponent from "./CreatePost";
import CardGroupComponent from "./CardGroup";
import {Stack} from "react-bootstrap";
import SearchForm from "./SearchForm";


function ContentContainerComponent() {


    return (
        <div>
            <SearchForm/>
            <Stack className="p-3 d-flex align-items-center justify-content-center" gap={3}>
                <div className="col-7"><CreatPostComponent/></div>
                <div className="col-7"><CardGroupComponent/></div>
            </Stack>
        </div>
    )
}

export default ContentContainerComponent;