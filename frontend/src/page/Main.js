import '../App.css';
import NavigationComponent from "../component/Navigation";
import ContentContainerComponent from "../component/ContentContainer";

function Main() {
  return (
    <div className="App">
      <NavigationComponent/>
      <ContentContainerComponent/>
    </div>
  );
}

export default Main;
