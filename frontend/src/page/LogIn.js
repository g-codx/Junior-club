import '../App.css';
import NavigationComponent from "../component/Navigation";
import LogInForm from "../component/LogInForm";


function LogIn() {
    return (
        <div className="App">
            <NavigationComponent/>
            <LogInForm/>
        </div>
    );
}

export default LogIn;