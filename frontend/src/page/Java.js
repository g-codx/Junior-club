import '../App.css';
import NavigationComponent from "../component/Navigation";
import CardInfoComponent from "../component/CardInfo";


function Java() {
    return (
        <div className="App">
            <NavigationComponent/>
            <CardInfoComponent section={"java"}/>
        </div>
    );
}

export default Java;