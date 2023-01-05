import '../App.css';
import NavigationComponent from "../component/Navigation";
import CardInfoComponent from "../component/CardInfo";


function Rust() {
    return (
        <div className="App">
            <NavigationComponent/>
            <CardInfoComponent section={"rust"}/>
        </div>
    );
}

export default Rust;